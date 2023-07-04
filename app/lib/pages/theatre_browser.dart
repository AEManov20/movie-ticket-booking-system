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

Future<Position> _determinePosition() async {
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
  return await Geolocator.getCurrentPosition();
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

  Future<Position>? location;

  Future<List<TheatreScreeningEvent>?> _fetchScreenings(
      String theatreId, DateTime chosenDate) async {
    return HandlerstheatrescreeningApi(ApiClient(basePath: baseApiPath))
        .getTimeline(theatreId, chosenDate,
            endDate: chosenDate.add(const Duration(days: 1)));
  }

  Future<List<ExtendedTheatre>?> _fetchNearbyTheatres(Position location) async {
    return HandlerstheatreApi(ApiClient(basePath: baseApiPath))
        .getNearby(location.latitude, location.longitude);
  }

  Future<List<ExtendedTheatre>?> _queryTheatresByName(String name) {
    return HandlerstheatreApi(ApiClient(basePath: baseApiPath))
        .searchByName(name);
  }

  @override
  void initState() {
    super.initState();
    location = _determinePosition();
  }

  @override
  void dispose() {
    super.dispose();

    mapController.dispose();
  }

  Widget _widgetWithPadding(Widget child) {
    return Padding(padding: const EdgeInsets.all(10), child: child);
  }

  Widget _theatresGrid(BuildContext context, List<ExtendedTheatre> list) {
    var messenger = ScaffoldMessenger.of(context);

    return GridView.count(
        crossAxisCount: 2,
        children: list
            .map((e) => GestureDetector(
                onLongPress: () {
                  var data =
                      ClipboardData(text: "${e.locationLon}, ${e.locationLat}");
                  Clipboard.setData(data);

                  messenger.clearSnackBars();
                  messenger.showSnackBar(const SnackBar(
                      content: Text("Copied location to clipboard!")));
                },
                onTap: () {
                  setState(() {
                    chosenTheatre = e;
                    screeningTimeline =
                        _fetchScreenings(chosenTheatre!.id, DateTime.now());
                  });
                },
                child: Card(
                    color: Colors.grey[900],
                    child: Column(children: [
                      ListTile(title: Text(e.name)),
                      ListTile(
                          leading: const Icon(Icons.airplane_ticket),
                          title:
                              Text("${e.ticketsCount.toString()} ticket(s)")),
                      ListTile(
                          leading: const Icon(Icons.room),
                          title: Text("${e.hallsCount} hall(s)")),
                      ListTile(
                          leading: const Icon(Icons.local_movies),
                          title: Text("${e.screeningsCount} screening(s)"))
                    ]))))
            .toList());
  }

  Widget _theatreView(BuildContext context, Position location) {
    return FlutterMap(
      options: MapOptions(
        center: LatLng(location.latitude, location.longitude),
        zoom: 9.2,
      ),
      nonRotatedChildren: [
        RichAttributionWidget(
          attributions: [
            TextSourceAttribution(
              'OpenStreetMap contributors',
              onTap: () =>
                  launchUrl(Uri.parse('https://openstreetmap.org/copyright')),
            ),
          ],
        ),
      ],
      children: [
        TileLayer(
          urlTemplate: 'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
        ),
      ],
    );
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
              onPageChanged: (index, reason) =>
                  setState(() => selectedScreeningIndex = index),
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
                    color: Colors.grey[900]!,
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
                selectionColor: Colors.grey[900]!,
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
        Divider(color: Colors.grey[900]!),
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

  Future<Position?> _showMapPopup(BuildContext context) async {
    var size = MediaQuery.of(context).size;

    return showDialog<Position>(
      context: context,
      barrierDismissible: false, // user must tap button!
      builder: (BuildContext context) {
        return AlertDialog(
          title: const Text('Pick a location'),
          content: SizedBox.fromSize(
              size: Size(size.width - 30, size.height - 30),
              child: FlutterMap(
                options: MapOptions(
                  zoom: 9.2,
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
                    urlTemplate:
                        'https://tile.openstreetmap.org/{z}/{x}/{y}.png',
                  ),
                  MarkerLayer(
                    markers: [
                      Marker(
                        point: LatLng(30, 40),
                        width: 80,
                        height: 80,
                        builder: (context) => FlutterLogo(),
                      ),
                    ],
                  ),
                ],
              )),
          actions: <Widget>[
            TextButton(
              child: const Text('Pick'),
              onPressed: () {
                Navigator.of(context).pop();
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
      return FutureBuilder(
        future: location,
        builder: (context, snapshot) {
          if (snapshot.hasData && snapshot.data != null) {
            return _theatreView(context, snapshot.data!);
          } else {
            WidgetsBinding.instance.addPostFrameCallback((timeStamp) {
              _showMapPopup(context);
            });
            return Container();
          }
        },
      );
      // return _theatreView(context);
    } else {
      return _screeningView(context);
    }
  }
}
