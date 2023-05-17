import 'package:http/http.dart' as http;
import 'package:flutter/material.dart';
import 'package:internship_app/user_post.dart';

class PostView extends StatefulWidget {
  const PostView({super.key});

  @override
  State<StatefulWidget> createState() => _PostViewState();
}

class _PostViewState extends State<PostView> {
  @override
  Widget build(BuildContext context) {
    return ListView(
      children: [
        UserPost(
            firstName: "Alexander",
            lastName: "Manov",
            bigTitle: "title",
            postContent:
                "Lorem Ipsum is simply dummy text of the printing and typesetting industry. Lorem Ipsum has been the industry's standard dummy text ever since the 1500s, when an unknown printer took a galley of type and scrambled it to make a type specimen book.",
            datePosted: DateTime.now(),
            accentColor: Colors.orange.shade300),
      ],
    );
  }
}
