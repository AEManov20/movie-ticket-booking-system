import 'package:api/api.dart';
import 'package:carousel_slider/carousel_slider.dart';
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
  // TODO: please remember to update this
  bool chosenTheatreDidUpdate = false;
  Future<Theatre?>? chosenTheatre;
  DateTime? chosenDate;

  Future<List<Theatre>?>? searchTheatres;
  Future<List<Theatre>?>? nearbyTheatres;
  Future<List<TheatreScreeningEvent>?>? screeningTimeline;

  CarouselController buttonCarouselController = CarouselController();

  // HandlerstheatreApi(ApiClient(basePath: baseApiPath))
  //       .searchByName(name);

  Future<List<Theatre>?> _fetchNearbyTheatres() async {
    var location = await _determinePosition();
    return HandlerstheatreApi(ApiClient(basePath: baseApiPath))
        .getNearby(location.longitude, location.latitude);
  }

  @override
  void initState() {
    super.initState();

    nearbyTheatres = _fetchNearbyTheatres();
  }

  @override
  void didUpdateWidget(covariant TheatreBrowserPage oldWidget) async {
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
    return Padding(padding: EdgeInsets.all(10), child: child);
  }

  Widget _theatreView() {
    return Column(
      children: [
        _widgetWithPadding(TextField(
          decoration: InputDecoration(
              border: const OutlineInputBorder(), label: Text("Search movies")),
        )),
        Expanded(child: Center(child: Text("Nearby theatres view")))
      ],
    );
  }

  Widget _screeningView() {
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
                    borderRadius: BorderRadius.all(Radius.circular(10))),
                margin: EdgeInsets.all(5),
              ))),
              Expanded(
                  child: _widgetWithPadding(Row(
                children: [
                  for (int i = 0; i < 6; i++)
                    Flexible(
                      child: Container(
                        decoration: BoxDecoration(
                            color: Colors.grey[900]!,
                            borderRadius:
                                BorderRadius.all(Radius.circular(10))),
                        margin: EdgeInsets.all(5),
                      ),
                    )
                ],
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
                    carouselController: buttonCarouselController,
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
      return _theatreView();
    } else {
      return _screeningView();
    }
  }
}
