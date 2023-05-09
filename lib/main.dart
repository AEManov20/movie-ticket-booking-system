import 'package:flutter/material.dart';

void main() {
  runApp(const MainApp());
}

class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        title: 'test',
        theme: ThemeData(primaryColor: Colors.amber),
        home: const HomePage());
  }
}

class HomePage extends StatefulWidget {
  const HomePage({super.key});

  @override
  State<HomePage> createState() => _HomePageState();
}

class _HomePageState extends State<HomePage> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text("Hello world"),
      ),
      body: Container(
          child: ListView(
        children: [
          Card(
            margin: const EdgeInsets.all(10.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              children: [
                Container(
                    padding: const EdgeInsets.all(16.0),
                    child: Row(
                      children: [
                        const CircleAvatar(child: Text("H")),
                        Container(margin: EdgeInsets.fromLTRB(15.0, 0.0, 0.0, 0.0), child: Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: const [
                            Text(
                              "Hristo Botev",
                            ),
                            Text("September 27, 2016")
                          ],
                        ))
                      ],
                    )),
                Container(
                    padding: const EdgeInsets.all(50.0),
                    decoration: const BoxDecoration(color: Colors.blue),
                    child: const Text("Card",
                        style: TextStyle(
                            color: Colors.white,
                            fontWeight: FontWeight.bold,
                            fontSize: 50))),
                Container(
                  padding: const EdgeInsets.all(15.0),
                  child: const Text(
                      "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s"),
                )
              ],
            ),
          )
        ],
      )),
    );
  }
}
