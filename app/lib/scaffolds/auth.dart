import 'package:flutter/material.dart';
import 'package:internship_app/widgets/auth.dart';

class AuthScaffold extends StatelessWidget {
  const AuthScaffold({super.key});

  @override
  Widget build(BuildContext context) {
    return const Scaffold(body: Center(child: AuthButtons()));
  }
}
