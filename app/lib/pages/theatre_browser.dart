import 'package:carousel_slider/carousel_slider.dart';
import 'package:flutter/material.dart';

class TheatreBrowserPage extends StatefulWidget {
  const TheatreBrowserPage({super.key});

  @override
  State<TheatreBrowserPage> createState() => _TheatreBrowserPageState();
}

class _TheatreBrowserPageState extends State<TheatreBrowserPage> {
  String? chosenTheatreId;
  DateTime? chosenDate;
  CarouselController buttonCarouselController = CarouselController();

  Widget _widgetWithPadding(Widget child) {
    return Padding(padding: EdgeInsets.all(10), child: child);
  }

  Widget _theatreView() {
    return Container();
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
                      Expanded(
                          child: Container(
                              decoration: BoxDecoration(
                                  color: Colors.grey[900]!,
                                  borderRadius:
                                      BorderRadius.all(Radius.circular(30)))))
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
    if (chosenTheatreId != null) {
      return _theatreView();
    } else {
      return _screeningView();
    }
  }
}
