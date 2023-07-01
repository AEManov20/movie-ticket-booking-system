import 'package:api/api.dart';
import 'package:async/async.dart';
import 'package:carousel_slider/carousel_slider.dart';
import 'package:date_picker_timeline/date_picker_widget.dart';
import 'package:flutter/material.dart';
import 'package:internship_app/main.dart';
import 'package:geolocator/geolocator.dart';

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

class TheatreBrowserPage extends StatefulWidget {
  const TheatreBrowserPage({super.key});

  @override
  State<TheatreBrowserPage> createState() => _TheatreBrowserPageState();
}

class _TheatreBrowserPageState extends State<TheatreBrowserPage> {
  ExtendedTheatre? chosenTheatre;
  DateTime? chosenDate;

  CancelableOperation<List<ExtendedTheatre>?>? searchTheatres;
  Future<List<ExtendedTheatre>?>? nearbyTheatres;
  Future<List<TheatreScreeningEvent>?>? screeningTimeline;

  CarouselController movieCarouselController = CarouselController();
  CarouselController dateCarouselController = CarouselController();

  Future<List<TheatreScreeningEvent>?> _fetchScreenings(
      String theatreId) async {
    return HandlerstheatrescreeningApi(ApiClient(basePath: baseApiPath))
        .getTimeline(theatreId, DateTime.now());
  }

  Future<List<ExtendedTheatre>?> _fetchNearbyTheatres() async {
    var location = await _determinePosition();
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

    nearbyTheatres = _fetchNearbyTheatres();
  }

  @override
  void didUpdateWidget(covariant TheatreBrowserPage oldWidget) {
    super.didUpdateWidget(oldWidget);
    // if (chosenTheatreId != null && chosenTheatreDidUpdate) {
    //   if (chosenDate != null) {
    //     HandlerstheatrescreeningApi(ApiClient(basePath: baseApiPath))
    //         .getTimeline(chosenTheatreId!, chosenDate!)
    //         .then(print)
    //         .onError((error, stackTrace) {
    //       print(error);
    //       print(stackTrace);
    //     });
    //   } else {
    //     HandlerstheatrescreeningApi(ApiClient(basePath: baseApiPath))
    //         .getTimeline(chosenTheatreId!, DateTime.now())
    //         .then(print)
    //         .onError((error, stackTrace) {
    //       print(error);
    //       print(stackTrace);
    //     });
    //   }
    // }
  }

  Widget _widgetWithPadding(Widget child) {
    return Padding(padding: const EdgeInsets.all(10), child: child);
  }

  Widget _theatresGrid(List<ExtendedTheatre> list) {
    return GridView.count(
        crossAxisCount: 2,
        children: list
            .map((e) => GestureDetector(
                onTap: () {
                  setState(() {
                    chosenTheatre = e;
                    screeningTimeline = _fetchScreenings(chosenTheatre!.id);
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

  Widget _theatreView(BuildContext context) {
    return Column(
      children: [
        _widgetWithPadding(TextField(
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
                var cancelled = false;

                // debouncing magic with CancelableOperation :)
                searchTheatres = CancelableOperation.fromFuture(() async {
                  await Future.delayed(const Duration(milliseconds: 250));
                  if (cancelled) return <ExtendedTheatre>[];
                  return await _queryTheatresByName(value);
                }(), onCancel: () {
                  cancelled = true;
                });
              });
            }
          },
          decoration: const InputDecoration(
              border: OutlineInputBorder(), label: Text("Search theatres")),
        )),
        Expanded(child: _widgetWithPadding((() {
          if (searchTheatres != null) {
            return FutureBuilder(
              future: searchTheatres!.value,
              builder: (context, snapshot) {
                if (snapshot.hasData && snapshot.data != null) {
                  return _theatresGrid(snapshot.data!);
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
              },
            );
          } else if (nearbyTheatres != null) {
            return FutureBuilder<List<ExtendedTheatre>?>(
                future: nearbyTheatres,
                builder: (context, snapshot) {
                  if (snapshot.hasData && snapshot.data != null) {
                    return _theatresGrid(snapshot.data!);
                  } else if (snapshot.hasError) {
                    return Center(
                      child: Column(
                        children: [
                          const Icon(Icons.location_disabled),
                          Center(child: Text(snapshot.error.toString()))
                        ],
                      ),
                    );
                  } else {
                    return Container();
                  }
                });
          } else {
            return const Center(
                child: Text("Nearby Theatres future not present."));
          }
        })()))
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
                    const TextStyle(fontSize: 30, color: Colors.white),
                monthTextStyle:
                    const TextStyle(fontSize: 10, color: Colors.white),
                selectionColor: Colors.grey[900]!,
                // exactly three weeks
                daysCount: 7 * 3,
                // i don't even know why i'm subtracting from 66
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
            child: Column(
              children: [
                _widgetWithPadding(Text("MOVIE TITLE",
                    style:
                        TextStyle(fontSize: 30, fontWeight: FontWeight.bold))),
                Expanded(
                    child: Padding(
                  padding: EdgeInsets.fromLTRB(0, 10, 0, 10),
                  child: CarouselSlider(
                    items: [
                      Container(
                          decoration: BoxDecoration(
                              color: Colors.grey[900]!,
                              borderRadius:
                                  BorderRadius.all(Radius.circular(30))))
                    ],
                    carouselController: movieCarouselController,
                    options: CarouselOptions(
                      autoPlay: false,
                      enlargeCenterPage: true,
                      viewportFraction: 0.9,
                      aspectRatio: 4 / 5,
                      initialPage: 2,
                    ),
                  ),
                ))
              ],
            ))
      ],
    );
  }

  @override
  Widget build(BuildContext context) {
    if (chosenTheatre == null) {
      return _theatreView(context);
    } else {
      return _screeningView(context);
    }
  }
}
