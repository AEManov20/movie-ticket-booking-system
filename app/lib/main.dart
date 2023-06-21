import 'package:flutter/material.dart';
import 'package:google_nav_bar/google_nav_bar.dart';
import 'package:internship_app/post_view.dart';
import 'package:openapi_generator_annotations/openapi_generator_annotations.dart';

Future<void> main() async {
  runApp(const MainApp());
}

@Openapi(
  additionalProperties:
      AdditionalProperties(pubName: 'api', pubAuthor: 'Alexander Manov'),
  inputSpecFile: 'http://localhost:8080/api-docs/openapi.json',
  generatorName: Generator.dart,
  outputDirectory: './api',
  runSourceGenOnOutput: true,
  alwaysRun: true,
)
class MainApp extends StatelessWidget {
  const MainApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        theme: ThemeData(primaryColor: Colors.amber), home: const HomePage());
  }
}

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: PostView(),
      bottomNavigationBar: GNav(tabs: [
        GButton(icon: Icons.favorite),
        GButton(icon: Icons.account_box),
      ]),
    );
  }
}
