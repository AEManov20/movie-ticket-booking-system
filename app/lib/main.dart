import 'package:flutter/material.dart';
import 'package:google_nav_bar/google_nav_bar.dart';
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
          fontFamily: "FiraSans",
          colorSchemeSeed: const Color(0xff6750a4),
          useMaterial3: true,
        ),
        themeMode: ThemeMode.dark,
        home: AuthPage());
  }
}

class AuthPage extends StatelessWidget {
  const AuthPage({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(body: Center(child: AuthButtons()));
  }
}

class AuthButtons extends StatefulWidget {
  const AuthButtons({super.key});

  @override
  State<AuthButtons> createState() => _AuthButtonsState();
}

class _AuthButtonsState extends State<AuthButtons> {
  int pageState = 0;

  Widget _styledAuthButton(String text, void Function() callback) {
    return Container(
        margin: const EdgeInsets.symmetric(horizontal: 30, vertical: 5),
        child: FilledButton(
          onPressed: callback,
          style: ButtonStyle(
              shape: MaterialStateProperty.all(RoundedRectangleBorder(
                  borderRadius: BorderRadius.circular(3.0))),
              padding: MaterialStateProperty.all(const EdgeInsets.all(20.0))),
          child: Text(text),
        ));
  }

  Widget _styledTextField(String label, bool isSecret) {
    return Container(
        margin: const EdgeInsets.fromLTRB(30, 5, 30, 5),
        child: TextFormField(
          decoration: InputDecoration(
              border: const OutlineInputBorder(), label: Text(label)),
        ));
  }

  @override
  Widget build(BuildContext context) {
    List<Widget> children;

    switch (pageState) {
      case 0:
        children = [
          const Center(child: Text("Welcome", style: TextStyle(fontSize: 60))),
          const Divider(),
          _styledAuthButton("Login", () {
            setState(() {
              pageState = 1;
            });
          }),
          _styledAuthButton("Register", () {
            setState(() {
              pageState = 2;
            });
          }),
          _styledAuthButton("Forgot password", () {})
        ];
        break;
      case 1:
        children = [
          const Center(child: Text("Login", style: TextStyle(fontSize: 60))),
          const Divider(),
          _styledTextField("Username", false),
          _styledTextField("Password", true),
          _styledAuthButton("Login", () {}),
          _styledAuthButton("Back", () {
            setState(() {
              pageState = 0;
            });
          })
        ];
        break;
      case 2:
        children = [
          const Center(child: Text("Register", style: TextStyle(fontSize: 60))),
          const Divider(),
          _styledTextField("Email", false),
          _styledTextField("First Name", false),
          _styledTextField("Last Name", false),
          _styledTextField("Username", false),
          _styledTextField("Password", true),
          _styledAuthButton("Register", () {}),
          _styledAuthButton("Back", () {
            setState(() {
              pageState = 0;
            });
          })
        ];
        break;
      default:
        throw StateError("Invalid page state");
    }

    return Column(
      mainAxisAlignment: MainAxisAlignment.center,
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: children,
    );
  }
}

class MainPage extends StatelessWidget {
  const MainPage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(),
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
