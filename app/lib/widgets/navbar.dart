import 'package:flutter/material.dart';
import 'package:google_nav_bar/google_nav_bar.dart';

class NavBar extends StatelessWidget {
  final ValueChanged<int>? onTabChange;
  final int selectedIndex;

  const NavBar({super.key, this.onTabChange, this.selectedIndex = 0});

  @override
  Widget build(BuildContext context) {
    return SafeArea(
        child: Container(
      color: Colors.black,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 15.0, vertical: 8),
        child: GNav(
            rippleColor: Colors.black26,
            hoverColor: Colors.black12,
            gap: 8,
            activeColor: Colors.white,
            selectedIndex: selectedIndex,
            iconSize: 24,
            padding: const EdgeInsets.symmetric(horizontal: 20, vertical: 12),
            duration: const Duration(milliseconds: 200),
            tabBackgroundColor: Color(0xff111111),
            color: Colors.white,
            onTabChange: onTabChange,
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
                icon: Icons.add_circle,
                text: 'Tickets',
              ),
              GButton(
                icon: Icons.account_box,
                text: 'Profile',
              )
            ]),
      ),
    ));
  }
}
