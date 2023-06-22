import 'package:flutter/material.dart';
import 'package:google_nav_bar/google_nav_bar.dart';
import 'package:internship_app/pages/login.dart';
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
        theme: ThemeData(
            colorSchemeSeed: const Color(0xff6750a4), useMaterial3: true),
        themeMode: ThemeMode.dark,
        home: AuthPage(
          buttons: [
            ButtonData(text: "Login", callback: () => {}),
            ButtonData(text: "Register", callback: () => {}),
            ButtonData(text: "Forgot password?", callback: () => {}),
          ],
        ));
  }
}

class ButtonData {
  String text;
  void Function() callback;

  ButtonData({required this.text, required this.callback});
}

class AuthPage extends StatelessWidget {
  final List<ButtonData> buttons;

  const AuthPage({super.key, required this.buttons});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
        body: Center(
            child: Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        const Center(child: Text("Welcome", style: TextStyle(fontSize: 60))),
        ...buttons.map((e) => FilledButton(
              onPressed: e.callback,
              child: Text(e.text),
            ))
      ],
    )));
  }
}

class MainPage extends StatelessWidget {
  const MainPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: const LoginPage(),
      bottomNavigationBar: Container(
          decoration: BoxDecoration(
            color: Colors.white,
            boxShadow: [
              BoxShadow(
                blurRadius: 20,
                color: Colors.black.withOpacity(.1),
              )
            ],
          ),
          child: const NavBar()),
    );
  }
}

class NavBar extends StatefulWidget {
  const NavBar({super.key});

  @override
  State<StatefulWidget> createState() => _NavBarState();
}

class _NavBarState extends State<NavBar> {
  _NavBarState();

  @override
  Widget build(BuildContext context) {
    return SafeArea(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 15.0, vertical: 8),
        child: GNav(
            rippleColor: Colors.grey[300]!,
            hoverColor: Colors.grey[100]!,
            gap: 8,
            activeColor: Colors.black,
            iconSize: 24,
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            duration: const Duration(milliseconds: 200),
            tabBackgroundColor: Colors.grey[100]!,
            color: Colors.black,
            tabs: const [
              GButton(
                icon: Icons.movie,
                text: 'Movies',
              ),
              GButton(
                icon: Icons.theaters,
                text: 'Theatres',
              ),
              GButton(
                icon: Icons.search,
                text: 'Search',
              ),
              GButton(
                icon: Icons.account_box,
                text: 'Profile',
              )
            ]),
      ),
    );
  }
}
