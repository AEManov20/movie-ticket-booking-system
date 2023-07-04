import 'dart:ui';

import 'package:api/api.dart';
import 'package:async/async.dart';
import 'package:carousel_slider/carousel_slider.dart';
import 'package:date_picker_timeline/date_picker_widget.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_map/flutter_map.dart';
import 'package:latlong2/latlong.dart';
import 'package:internship_app/main.dart';
import 'package:geolocator/geolocator.dart';
import 'package:intl/intl.dart';
import 'package:url_launcher/url_launcher.dart';

Future<LatLng> _determinePosition() async {
  bool serviceEnabled;
  LocationPermission permission;

  // Test if location services are enabled.
  serviceEnabled = await Geolocator.isLocationServiceEnabled();
  if (!serviceEnabled) {
    // Location services are not enabled don't continue
    // accessing the position and request users of the
    // App to enable the location services.
    return Future.error(Exception('Location services are disabled.'));
  }

  permission = await Geolocator.checkPermission();
  if (permission == LocationPermission.denied) {
    permission = await Geolocator.requestPermission();
    if (permission == LocationPermission.denied) {
      // Permissions are denied, next time you could try
      // requesting permissions again (this is also where
      // Android's shouldShowRequestPermissionRationale
      // returned true. According to Android guidelines
      // your App should show an explanatory UI now.
      return Future.error(Exception('Location permissions are denied'));
    }
  }

  if (permission == LocationPermission.deniedForever) {
    // Permissions are denied forever, handle appropriately.
    return Future.error(Exception(
        'Location permissions are permanently denied, we cannot request permissions.'));
  }

  // When we reach here, permissions are granted and we can
  // continue accessing the position of the device.
  Position res = await Geolocator.getCurrentPosition();
  return LatLng(res.latitude, res.longitude);
}

CancelableOperation<T> _debounceFuture<T>(
    Future<T> Function() callback, T cancelValue,
    [Duration delay = const Duration(milliseconds: 350)]) {
  var cancelled = false;
  return CancelableOperation.fromFuture(() async {
    await Future.delayed(delay);
    if (cancelled) {
      return cancelValue;
    } else {
      return await callback();
    }
  }(), onCancel: () {
    cancelled = true;
  });
}

class TheatreBrowserPage extends StatefulWidget {
  const TheatreBrowserPage({super.key});

  @override
  State<TheatreBrowserPage> createState() => _TheatreBrowserPageState();
}

class _TheatreBrowserPageState extends State<TheatreBrowserPage> {
  ExtendedTheatre? chosenTheatre;
  int selectedScreeningIndex = 0;

  CancelableOperation<List<ExtendedTheatre>?>? searchTheatres;
  Future<List<ExtendedTheatre>?>? nearbyTheatres;
  Future<List<TheatreScreeningEvent>?>? screeningTimeline;

  CarouselController movieCarouselController = CarouselController();
  MapController mapController = MapController();

  Future<LatLng?>? location;

  Image? previousBg;
  Image? currentBg;

  Future<List<TheatreScreeningEvent>?> _fetchScreenings(
      String theatreId, DateTime chosenDate) async {
    return HandlerstheatrescreeningApi(ApiClient(basePath: baseApiPath))
        .getTimeline(theatreId, chosenDate,
            endDate: chosenDate.add(const Duration(days: 1)));
  }

  Future<List<ExtendedTheatre>?> _fetchNearbyTheatres(LatLng location) async {
    return HandlerstheatreApi(ApiClient(basePath: baseApiPath))
        .getNearby(location.longitude, location.latitude);
  }

  Future<List<ExtendedTheatre>?> _queryTheatresByName(String name) {
    return HandlerstheatreApi(ApiClient(basePath: baseApiPath))
        .searchByName(name);
  }

  @override
  void initState() {
    super.initState();
    location = _determinePosition();

    () async {
      try {
        var res = await location;
        if (res != null) {
          setState(() {
            nearbyTheatres = _fetchNearbyTheatres(res);
          });
        }
      } on Exception catch (_) {}
    }();
  }

  @override
  void dispose() {
    super.dispose();

    mapController.dispose();
  }

  Widget _widgetWithPadding(Widget child) {
    return Padding(padding: const EdgeInsets.all(10), child: child);
  }

  Widget _theatreMarkers(BuildContext context, List<ExtendedTheatre>? list) {
    return MarkerLayer(
      markers: list != null
          ? list
              .map((e) => Marker(
                  anchorPos: AnchorPos.align(AnchorAlign.bottom),
                  point: LatLng(e.locationLat, e.locationLon),
                  builder: (ctx) => GestureDetector(
                      onTap: () => setState(() {
                            chosenTheatre = e;
                          }),
                      onLongPress: () => _showTheatreModalSheet(e),
                      child: const Icon(Icons.location_on, color: Colors.red))))
              .toList()
          : [],
    );
  }

  Widget _theatreView(LatLng? location) {
    return FutureBuilder<List<ExtendedTheatre>?>(
        future: searchTheatres?.value ?? nearbyTheatres,
        builder: (context, snapshot) {
          if (snapshot.hasError) {
            return Center(
              child: Column(
                children: [
                  const Icon(Icons.location_disabled),
                  Center(child: Text(snapshot.error.toString()))
                ],
              ),
            );
          }
          return FlutterMap(
            options: MapOptions(
              center: location,
            ),
            nonRotatedChildren: [
              RichAttributionWidget(
                attributions: [
                  TextSourceAttribution(
                    'OpenStreetMap contributors',
                    onTap: () => launchUrl(
                        Uri.parse('https://openstreetmap.org/copyright')),
                  ),
                ],
              ),
            ],
            children: [
              TileLayer(
                urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
              ),
              _theatreMarkers(context, snapshot.data)
            ],
          );
        });
  }

  Widget _screeningInfo(
      BuildContext context, List<TheatreScreeningEvent>? screeningTimeline) {
    if (screeningTimeline == null || screeningTimeline.isEmpty) {
      return const Center(child: Text("No screenings"));
    }

    return Column(
      children: [
        Center(
            child: _widgetWithPadding(Text(
                screeningTimeline[selectedScreeningIndex].movieName,
                style: const TextStyle(
                    fontSize: 30, fontWeight: FontWeight.bold)))),
        Wrap(direction: Axis.horizontal, children: [
          const Icon(Icons.calendar_month),
          const SizedBox(width: 5),
          Text(DateFormat('hh:mm').format(
              screeningTimeline[selectedScreeningIndex]
                  .startingTime
                  .toLocal())),
          const SizedBox(width: 50),
          const Icon(Icons.access_time),
          const SizedBox(width: 5),
          Text("${screeningTimeline[selectedScreeningIndex].length} min"),
          const SizedBox(width: 50),
          const Icon(Icons.chair),
          const SizedBox(width: 5),
          Text(screeningTimeline[selectedScreeningIndex].hallName),
        ]),
        Expanded(
            child: Padding(
          padding: const EdgeInsets.fromLTRB(0, 10, 0, 10),
          child: CarouselSlider(
            items: screeningTimeline
                .map((e) => e.moviePosterUrl == null
                    ? Container(
                        decoration: BoxDecoration(
                            color: Colors.grey[900]!,
                            borderRadius:
                                const BorderRadius.all(Radius.circular(30))))
                    : ClipRRect(
                        borderRadius:
                            const BorderRadius.all(Radius.circular(30)),
                        child: Image.network(e.moviePosterUrl!,
                            fit: BoxFit.fitHeight)))
                .toList(),
            carouselController: movieCarouselController,
            options: CarouselOptions(
              onPageChanged: (index, reason) => setState(() {
                previousBg = currentBg;
                selectedScreeningIndex = index;
              }),
              autoPlay: false,
              enlargeCenterPage: true,
              viewportFraction: 0.9,
              aspectRatio: 4 / 5,
              initialPage: 2,
            ),
          ),
        ))
      ],
    );
  }

  Widget _screeningView(BuildContext context) {
    return Column(
      children: [
        Flexible(
          flex: 1,
          child: Column(
            children: [
              Expanded(
                  child: _widgetWithPadding(Container(
                decoration: BoxDecoration(
                    color: Colors.black,
                    borderRadius: const BorderRadius.all(Radius.circular(10))),
              ))),
              Expanded(
                  child: _widgetWithPadding(DatePicker(
                DateTime.now(),
                initialSelectedDate: DateTime.now(),
                dayTextStyle:
                    const TextStyle(fontSize: 10, color: Colors.white),
                dateTextStyle:
                    const TextStyle(fontSize: 25, color: Colors.white),
                monthTextStyle:
                    const TextStyle(fontSize: 10, color: Colors.white),
                selectionColor: Colors.black,
                // exactly three weeks
                daysCount: 7 * 3,
                onDateChange: (selectedDate) => setState(() {
                  selectedScreeningIndex = 0;
                  screeningTimeline =
                      _fetchScreenings(chosenTheatre!.id, selectedDate);
                }),
                // i don't even know why i'm subtracting by 66
                width: (MediaQuery.of(context).size.width - 66) / 7,
                deactivatedColor: Colors.white,
                selectedTextColor: Colors.white,
              )))
            ],
          ),
        ),
        Flexible(
            flex: 3,
            child: FutureBuilder(
                future: screeningTimeline,
                builder: (context, snapshot) {
                  if (snapshot.hasData && snapshot.data != null) {
                    return _screeningInfo(context, snapshot.data);
                  } else if (snapshot.hasError) {
                    return Center(
                      child: Column(
                        children: [
                          const Icon(Icons.error),
                          Center(child: Text(snapshot.error.toString()))
                        ],
                      ),
                    );
                  } else {
                    return Container();
                  }
                }))
      ],
    );
  }

  Future<void> _showTheatreModalSheet(ExtendedTheatre theatre) {
    return showModalBottomSheet<void>(
      context: context,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(5.0),
      ),
      builder: (BuildContext context) {
        return Container(
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            mainAxisSize: MainAxisSize.min,
            children: <Widget>[
              ListTile(title: Text(theatre.name)),
              ListTile(
                  leading: const Icon(Icons.airplane_ticket),
                  title: Text("${theatre.ticketsCount.toString()} ticket(s)")),
              ListTile(
                  leading: const Icon(Icons.room),
                  title: Text("${theatre.hallsCount} hall(s)")),
              ListTile(
                  leading: const Icon(Icons.local_movies),
                  title: Text("${theatre.screeningsCount} screening(s)")),
              GestureDetector(
                  onTap: () => setState(() {
                        chosenTheatre = theatre;
                      }),
                  child: const ListTile(
                      leading: Icon(Icons.back_hand),
                      title: Text("Check out screenings...")))
            ],
          ),
        );
      },
    );
  }

  Future<LatLng?> _showMapPopup() async {
    var size = MediaQuery.of(context).size;
    var controller = MapController();

    return showDialog<LatLng>(
      context: context,
      barrierDismissible: false, // user must tap button!
      builder: (BuildContext context) {
        return AlertDialog(
          title: const Text('Pick a location'),
          content: SizedBox.fromSize(
              size: Size(size.width - 30, size.height - 30),
              child: FlutterMap(
                mapController: controller,
                options: MapOptions(),
                nonRotatedChildren: [
                  RichAttributionWidget(
                    attributions: [
                      TextSourceAttribution(
                        'OpenStreetMap contributors',
                        onTap: () => launchUrl(
                            Uri.parse('https://openstreetmap.org/copyright')),
                      ),
                    ],
                  ),
                ],
                children: [
                  TileLayer(
                    urlTemplate:
                        'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
                  ),
                ],
              )),
          actions: <Widget>[
            TextButton(
              child: const Text('Pick'),
              onPressed: () {
                Navigator.of(context).pop(controller.center);
              },
            ),
          ],
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    if (chosenTheatre == null) {
      return Stack(children: [
        FutureBuilder(
            future: location,
            builder: (context, snapshot) {
              if (snapshot.hasError) {
                WidgetsBinding.instance.addPostFrameCallback((timeStamp) async {
                  var res = await _showMapPopup();

                  // pain. useEffect exists for a reason.
                  setState(() {
                    location = Future.value(res);

                    () async {
                      var res = await location;
                      setState(() {
                        if (res != null) {
                          nearbyTheatres = _fetchNearbyTheatres(res);
                        } else {
                          nearbyTheatres = null;
                        }
                      });
                    }();
                  });
                });
              }
              return _theatreView(snapshot.data);
            }),
        _widgetWithPadding(ClipRRect(
            borderRadius: const BorderRadius.all(Radius.circular(5.0)),
            child: TextField(
              onChanged: (value) {
                if (searchTheatres != null) {
                  setState(() {
                    searchTheatres!.cancel();
                  });
                }

                if (value.isEmpty) {
                  setState(() {
                    searchTheatres = null;
                  });
                } else {
                  setState(() {
                    searchTheatres = _debounceFuture(
                        () => _queryTheatresByName(value), List.empty());
                  });
                }
              },
              style: const TextStyle(color: Colors.black),
              decoration: const InputDecoration(
                  border: InputBorder.none,
                  fillColor: Colors.white,
                  filled: true,
                  labelStyle: TextStyle(color: Colors.grey),
                  label: Text("Search theatres")),
            ))),
      ]);
    } else {
      return FutureBuilder(
          future: screeningTimeline,
          builder: (context, snapshot) {
            if (snapshot.hasData &&
                snapshot.data != null &&
                snapshot.data!.isNotEmpty) {
              try {
                if (snapshot.data!
                        .elementAt(selectedScreeningIndex)
                        .moviePosterUrl !=
                    null) {
                  currentBg = Image.network(
                    snapshot.data![selectedScreeningIndex].moviePosterUrl!,
                    color: Color(0x99000000),
                    colorBlendMode: BlendMode.darken,
                    fit: BoxFit.cover,
                  );
                }
              } on Exception catch (_) {}
            }

            return Stack(children: [
              // Second image with animated opacity
              SizedBox.expand(child: currentBg ?? Container()),
              BackdropFilter(
                  filter: ImageFilter.blur(sigmaX: 50, sigmaY: 50),
                  child: _screeningView(context)),
            ]);
          });
    }
  }
}
