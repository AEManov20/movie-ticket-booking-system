import 'package:flutter/material.dart';

class TicketsPage extends StatefulWidget {
  const TicketsPage({super.key});
  
  @override
  State<TicketsPage> createState() => _TicketsPageState();
}

class _TicketsPageState extends State<TicketsPage> {
  @override
  Widget build(BuildContext context) {
    return Center(child: Text("Tickets page"));
  }
}