import 'dart:async';
import 'dart:math';
import 'dart:ui';

import 'package:api/api.dart';
import 'package:flutter/material.dart';
import 'package:flutter/rendering.dart';
import 'package:hive/hive.dart';
import 'package:internship_app/main.dart';

String? _defaultValidator(String? val) {
  if (val == null || val.isEmpty) {
    return "This field is required!";
  }
  return null;
}

String? _rangeDefaultValidator(String? val, int min, int max) {
  if (val == null || val.isEmpty) {
    return _defaultValidator(val);
  } else if (val.length < min || val.length > max) {
    return "Field must be $min or below $max characters";
  } else {
    return null;
  }
}

class LoginForm extends StatefulWidget {
  final void Function(bool, bool)? notifyParent;
  final bool isLoading;

  const LoginForm({super.key, this.notifyParent, required this.isLoading});

  @override
  State<LoginForm> createState() => _LoginFormState();
}

class _LoginFormState extends State<LoginForm> {
  final _formKey = GlobalKey<FormState>();

  final _userNameController = TextEditingController();
  final _passwordController = TextEditingController();

  @override
  void dispose() {
    super.dispose();

    _userNameController.dispose();
    _passwordController.dispose();
  }

  void _submitHandler(BuildContext ctx) async {
    final messenger = ScaffoldMessenger.of(ctx);
    messenger.clearSnackBars();

    if (_formKey.currentState!.validate()) {
      messenger.showSnackBar(const SnackBar(content: Text("Processing...")));

      if (widget.notifyParent != null) {
        widget.notifyParent!(true, false);
      }

      try {
        var auth = await HandlersauthApi(ApiClient(basePath: baseApiPath))
            .loginUser(_userNameController.text, _passwordController.text);

        if (auth != null) {
          var authObj = OAuth(accessToken: auth.token);
          var userData = await HandlersuserApi(
                  ApiClient(basePath: baseApiPath, authentication: authObj))
              .getSelfUser();

          messenger.clearSnackBars();
          messenger.showSnackBar(
              const SnackBar(content: Text("Logged in! Hang on...")));

          var box = Hive.box(userBox);

          box.put('auth', auth.token);
          box.put('data', userData);
        } else {
          messenger.clearSnackBars();
          messenger.showSnackBar(
              const SnackBar(content: Text("Invalid credentials!")));
        }
      } on ApiException catch (e) {
        messenger.clearSnackBars();
        messenger.showSnackBar(SnackBar(content: Text("error: ${e.message}")));
        if (widget.notifyParent != null) {
          widget.notifyParent!(false, false);
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(
        children: [
          _AuthButtonsState._styledTextField(
              "Username",
              false,
              true,
              (val) => _rangeDefaultValidator(val, 8, 1000),
              _userNameController),
          _AuthButtonsState._styledTextField(
              "Password",
              true,
              true,
              (val) => _rangeDefaultValidator(val, 12, 64),
              _passwordController),
          _AuthButtonsState._styledAuthButton("Login",
              widget.isLoading ? null : () => _submitHandler(context), true),
        ],
      ),
    );
  }
}

class RegisterForm extends StatefulWidget {
  final void Function(bool, bool)? notifyParent;
  final bool isLoading;

  const RegisterForm({super.key, this.notifyParent, required this.isLoading});

  @override
  State<RegisterForm> createState() => _RegisterFormState();
}

class _RegisterFormState extends State<RegisterForm> {
  final _formKey = GlobalKey<FormState>();

  final _emailController = TextEditingController();
  final _firstNameController = TextEditingController();
  final _lastNameController = TextEditingController();
  final _userNameController = TextEditingController();
  final _passwordController = TextEditingController();

  @override
  void dispose() {
    super.dispose();

    _emailController.dispose();
    _firstNameController.dispose();
    _lastNameController.dispose();
    _userNameController.dispose();
    _passwordController.dispose();
  }

  void _submitHandler(BuildContext ctx) async {
    final messenger = ScaffoldMessenger.of(ctx);
    messenger.clearSnackBars();

    if (_formKey.currentState!.validate()) {
      messenger.showSnackBar(const SnackBar(content: Text("Processing...")));

      if (widget.notifyParent != null) {
        widget.notifyParent!(true, false);
      }

      try {
        await HandlersauthApi(ApiClient(basePath: 'http://localhost:8080'))
            .registerUser(FormUser(
                email: _emailController.text,
                firstName: _firstNameController.text,
                lastName: _lastNameController.text,
                password: _passwordController.text,
                username: _userNameController.text));

        messenger.clearSnackBars();
        messenger.showSnackBar(const SnackBar(
            content:
                Text("Registration successful! Please check your email.")));

        if (widget.notifyParent != null) {
          widget.notifyParent!(false, true);
        }
      } on ApiException catch (e) {
        messenger.clearSnackBars();
        messenger.showSnackBar(SnackBar(content: Text("error: ${e.message}")));

        if (widget.notifyParent != null) {
          widget.notifyParent!(false, false);
        }
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    return Form(
      key: _formKey,
      child: Column(children: [
        _AuthButtonsState._styledTextField("Email", false, true, (val) {
          if (val != null) {
            final bool emailValid = RegExp(
                    r"^[a-zA-Z0-9.a-zA-Z0-9.!#$%&'*+-/=?^_`{|}~]+@[a-zA-Z0-9]+\.[a-zA-Z]+")
                .hasMatch(val);

            if (!emailValid) return "Invalid email!";

            return _rangeDefaultValidator(val, 1, 150);
          } else {
            return _rangeDefaultValidator(val, 1, 150);
          }
        }, _emailController),
        _AuthButtonsState._styledTextField("First Name", false, true,
            (val) => _rangeDefaultValidator(val, 1, 50), _firstNameController),
        _AuthButtonsState._styledTextField("Last Name", false, true,
            (val) => _rangeDefaultValidator(val, 1, 50), _lastNameController),
        _AuthButtonsState._styledTextField("Username", false, true,
            (val) => _rangeDefaultValidator(val, 8, 50), _userNameController),
        _AuthButtonsState._styledTextField("Password", true, true,
            (val) => _rangeDefaultValidator(val, 12, 64), _passwordController),
        _AuthButtonsState._styledTextField("Repeat Password", true, true,
            (val) {
          if (val != _passwordController.text) {
            return "Passwords do not match!";
          }

          return null;
        }, null),
        _AuthButtonsState._styledAuthButton("Register",
            widget.isLoading ? null : () => _submitHandler(context), true),
      ]),
    );
  }
}

class AuthButtons extends StatefulWidget {
  const AuthButtons({super.key});

  @override
  State<AuthButtons> createState() => _AuthButtonsState();
}

class _AuthButtonsState extends State<AuthButtons> {
  int pageState = 0;
  bool isLoading = false;

  @override
  void initState() {
    super.initState();
  }

  static Widget _styledAuthButton(
      String text, void Function()? callback, bool withMargin) {
    Widget child = FilledButton(
      onPressed: callback,
      style: ButtonStyle(
          shape: MaterialStateProperty.all(
              RoundedRectangleBorder(borderRadius: BorderRadius.circular(3.0))),
          padding: MaterialStateProperty.all(const EdgeInsets.all(20.0))),
      child: Text(text),
    );

    if (withMargin) {
      return Container(
        width: double.infinity,
        margin: const EdgeInsets.symmetric(horizontal: 30, vertical: 5),
        child: child,
      );
    } else {
      return child;
    }
  }

  static Widget _styledTextField(String label, bool isSecret, bool withMargin,
      String? Function(String?)? validator, TextEditingController? controller) {
    Widget child = TextFormField(
      controller: controller,
      validator: validator,
      decoration: InputDecoration(
          border: const OutlineInputBorder(), label: Text(label)),
      obscureText: isSecret,
      enableSuggestions: !isSecret,
      autocorrect: !isSecret,
    );

    if (withMargin) {
      return Container(
          margin: const EdgeInsets.symmetric(horizontal: 30, vertical: 5),
          child: child);
    } else {
      return child;
    }
  }

  void _notifyLoading(bool state, bool returnToStart) {
    setState(() {
      if (returnToStart) {
        _goToHome();
      }
      isLoading = state;
    });
  }

  void _goToHome() {
    setState(() {
      pageState = 0;
    });
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
          }, true),
          _styledAuthButton("Register", () {
            setState(() {
              pageState = 2;
            });
          }, true),
          _styledAuthButton("Forgot password", null, true)
        ];
        break;
      case 1:
        children = [
          const Center(
              child: Text("Welcome back", style: TextStyle(fontSize: 60))),
          const Divider(),
          LoginForm(isLoading: isLoading, notifyParent: _notifyLoading),
          _styledAuthButton("Back", isLoading ? null : _goToHome, true)
        ];
        break;
      case 2:
        children = [
          const Center(child: Text("Register", style: TextStyle(fontSize: 60))),
          const Divider(),
          RegisterForm(isLoading: isLoading, notifyParent: _notifyLoading),
          _styledAuthButton("Back", isLoading ? null : _goToHome, true)
        ];
        break;
      default:
        throw StateError("Invalid page state");
    }

    return Stack(children: [
      SizedBox.expand(child: CustomPaint(foregroundPainter: ColorfulCircles())),
      BackdropFilter(
          filter: ImageFilter.blur(sigmaX: 20, sigmaY: 20),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.stretch,
            children: children,
          ))
    ]);
  }
}

class ColorfulCircles extends CustomPainter {
  var circleColors = [Colors.red, Colors.green, Colors.blue];

  @override
  void paint(Canvas canvas, Size size) {
    canvas.drawCircle(Offset(size.width - 200, size.height - 100), 25,
        Paint()..color = Colors.blue);

    for (int i = 0; i < circleColors.length; i++) {
      bool widthFlag = Random().nextBool();
      bool heightFlag = Random().nextBool();
      double dx = Random().nextDouble() * size.width;
      double dy = Random().nextDouble() * size.height;
      canvas.drawCircle(
          Offset(widthFlag ? size.width - dx : dx,
              heightFlag ? size.height - dy : dy),
          25,
          Paint()..color = circleColors[i]);
    }
  }

  @override
  SemanticsBuilderCallback get semanticsBuilder {
    return (Size size) {
      // Annotate a rectangle containing the picture of the sun
      // with the label "Sun". When text to speech feature is enabled on the
      // device, a user will be able to locate the sun on this picture by
      // touch.
      Rect rect = Offset.zero & size;
      final double width = size.shortestSide * 0.4;
      rect = const Alignment(0.8, -0.9).inscribe(Size(width, width), rect);
      return <CustomPainterSemantics>[
        CustomPainterSemantics(
          rect: rect,
          properties: const SemanticsProperties(
            label: 'Sun',
            textDirection: TextDirection.ltr,
          ),
        ),
      ];
    };
  }

  // Since this Sky painter has no fields, it always paints
  // the same thing and semantics information is the same.
  // Therefore we return false here. If we had fields (set
  // from the constructor) then we would return true if any
  // of them differed from the same fields on the oldDelegate.
  @override
  bool shouldRepaint(ColorfulCircles oldDelegate) => false;
  @override
  bool shouldRebuildSemantics(ColorfulCircles oldDelegate) => false;
}
