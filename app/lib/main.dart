import 'package:api/api.dart';
import 'package:flutter/material.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:internship_app/adapters/movie.dart';
import 'package:internship_app/adapters/theatre.dart';
import 'package:internship_app/adapters/user.dart';
import 'package:internship_app/pages/movie_browser.dart';
import 'package:internship_app/pages/self.dart';
import 'package:internship_app/pages/theatre_browser.dart';
import 'package:internship_app/pages/tickets.dart';
import 'package:internship_app/scaffolds/auth.dart';
import 'package:internship_app/widgets/navbar.dart';
import 'package:openapi_generator_annotations/openapi_generator_annotations.dart';

const favouriteMovieBox = 'favourite_movies_data';
const favouriteTheatreBox = 'favourite_theatres_data';
const userBox = 'user_data';
const settingsBox = 'settings_data';

const baseApiPath = 'http://localhost:8080';

Future<void> main() async {
  await Hive.initFlutter();

  Hive.registerAdapter(MovieAdapter());
  Hive.registerAdapter(TheatreAdapter());
  Hive.registerAdapter(UserAdapter());

  await Hive.openBox<Movie>(favouriteMovieBox);
  await Hive.openBox<Theatre>(favouriteTheatreBox);
  await Hive.openBox(userBox);
  await Hive.openBox(settingsBox);

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
        darkTheme: ThemeData(
            fontFamily: "FiraSans",
            colorScheme: const ColorScheme.dark(primary: Color(0xff6750a4)),
            useMaterial3: true),
        themeMode: ThemeMode.dark,
        home: ValueListenableBuilder(
          valueListenable: Hive.box(userBox).listenable(keys: ['auth']),
          builder: (context, value, child) {
            return value.get('auth') == null
                ? const AuthScaffold()
                : const MainPage();
          },
        ));
  }
}

class MovieCard extends StatelessWidget {
  const MovieCard({super.key});

  @override
  Widget build(BuildContext context) {
    return Card(
      margin: EdgeInsets.fromLTRB(0, 0, 0, 20),
      elevation: 5,
      child: SizedBox(width: 200, height: 300),
    );
  }
}

class MainPage extends StatefulWidget {
  const MainPage({super.key});

  @override
  State<MainPage> createState() => _MainPageState();
}

class _MainPageState extends State<MainPage> {
  int _pageIndex = 0;

  _updatePageIndex(int index) {
    setState(() {
      _pageIndex = index;
    });
  }

  Widget _pages() {
    switch (_pageIndex) {
      case 0:
        return MovieBrowserPage();
      case 1:
        return TheatreBrowserPage();
      case 2:
        return TicketsPage();
      case 3:
        return SelfPage();
      default:
        return Center(child: Text("404"));
    }
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: SizedBox(
          height: MediaQuery.of(context).size.height,
          width: MediaQuery.of(context).size.width,
          child: _pages()),
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
          child: NavBar(onTabChange: _updatePageIndex, selectedIndex: _pageIndex,)),
    );
  }
}
