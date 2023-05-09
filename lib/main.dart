import 'package:flutter/material.dart';
import 'package:internship_app/user_post.dart';

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
          UserPost(
              firstName: "Alexander",
              lastName: "Manov",
              bigTitle: "shite",
              postContent:
                  "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.",
              datePosted: DateTime.now(),
              accentColor: Colors.orange.shade300),
        ],
      )),
    );
  }
}
