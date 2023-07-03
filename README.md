# Movie Ticket Booking System

## Quick summary
The premise of this project is to simplify booking tickets for movies worldwide, to gather all the user interfaces made by different cinemas in one place. Except a booking system this is also a management system for cinemas including features like:
 - Managing booked tickets
 - System for checking booked tickets
 - Managing screenings
 - Managing layouts for cinema halls (seats and their pricings, if any)

## Problems with centralization
 - Problem: The current implementation of this system is centralised which could prove to be a problem in the future. The biggest problem being that if the central server goes down all of the instances of cinemas in the database go along with it. One possible solution to this problem is to divide the load on multiple server instances around the world with their own databases that they manage. But we can do better.
 - Possible solution (pt. 1): Description: Since cinemas already have servers, apps and/or sites of their own with which said users can book/reserve/buy tickets on and staff members can manage the cinema's screenings, tickets, hall layouts, etc on. The solution is to implement already laid out standards for interfaces, protocols and formats on these servers with which clients could communicate to cinemas' servers directly (primarily to exchange information about upcoming screenings and book/buy tickets for said screenings). Obliging them (cinema owners) to move to a different platform (the current centralised platform) where they would have to manage two instances of the cinema (one on their server and one on the centralised platform) would be a hassle both for the ticket buyer and the cinema owner.
 - Possible solution (pt. 2): Terms:
   - **Index Implementor** - A server whose purpose is to store information about cinemas and their outlets for communication
   - **Server Implementor** - A cinema's server that has implemented said interfaces, protocols and formats
   - **Client Implementor** - A UI implementation for communicating with **Server Implementors**
 - Possible solution (pt. 3): Steps of operation:
   - **Client Implementor** communicates with an **Index Implementor** to gather information about a cinema **Server Implementor**s (searching/querying can happen either by name, geolocation or other ways of identifying)
   ![clientindex-dark](./assets/client-index-dark.png#gh-dark-mode-only)
   ![clientindex-light](./assets/client-index-light.png#gh-light-mode-only)
   - Authorization with **Server Implementor** via agreed upon means.
   - **Client Implementor** communicates with a **Server Implementor** on which it is authenticated.
   ![clientserver-dark](./assets/client-server-dark.png#gh-dark-mode-only)
   ![clientserver-light](./assets/client-server-light.png#gh-light-mode-only)

