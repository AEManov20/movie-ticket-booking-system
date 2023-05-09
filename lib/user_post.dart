import 'package:flutter/material.dart';
import 'package:intl/intl.dart';

class UserPost extends StatelessWidget {
  const UserPost(
      {super.key,
      required this.firstName,
      required this.lastName,
      required this.bigTitle,
      required this.postContent,
      required this.datePosted,
      required this.accentColor});

  final String firstName, lastName, bigTitle, postContent;
  final Color accentColor;
  final DateTime datePosted;

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 5,
      margin: const EdgeInsets.all(10.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          Container(
              padding: const EdgeInsets.all(16.0),
              child: Row(
                children: [
                  CircleAvatar(
                      child: Text(firstName[0].toUpperCase().toString())),
                  Container(
                      margin: const EdgeInsets.fromLTRB(15.0, 0.0, 0.0, 0.0),
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            "$firstName $lastName",
                          ),
                          Text(DateFormat.yMMMMd('en-US').format(datePosted))
                        ],
                      ))
                ],
              )),
          Container(
              padding: const EdgeInsets.all(50.0),
              decoration: BoxDecoration(color: accentColor),
              child: Text(bigTitle,
                  style: TextStyle(
                      color: accentColor.computeLuminance() >= .5
                          ? Colors.black
                          : Colors.white,
                      fontWeight: FontWeight.bold,
                      fontSize: 50))),
          Container(
            padding: const EdgeInsets.all(15.0),
            child: Text(postContent),
          )
        ],
      ),
    );
  }
}
