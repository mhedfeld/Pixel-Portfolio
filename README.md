#Pixel Portfolio <br>
Backend in Rust ( Actix Web) , Tera, Sqlite 

----------------
   GENERAL
----------------

DISCLAIMER! 
THIS CODE IS NOT YET FINISHED AND WILL SHOW FLAWS AS SUCH!<br>
THERE MIGHT BE CODE THAT DOES NOTHING AT ALL, LEFT INSIDE!<br>

This is a basic Pixel Portfolio Page <br>
It consists of a Homepage, Aboutpage, Project Page (dynamic Project rendering), Contactpage, Greetpage and an Adminpage. <br>
<br>
<br>
<br>  

The Backend was built with the Rust Actix Web Framework <br>
<br>
Routes can be accessed via 127.0.0.1:8080/ <br>
                                 127.0.0.1:8080/about<br>
                                 127.0.0.1:8080/contact<br>
                                 127.0.0.1:8080/projects<br>
                                 127.0.0.1:8080/project/{id} (1, 3, 4, 5, 6) <br>
                                 127.0.0.1:8080/admin (login credentials in .env) <br>



--------------------
KNOWN CURRENT ISSUES
--------------------

I Changed the overall styling of the Page, just a day ago, as such the styling of the pages, might be buggy or not fully finished yet. <br>
Media Queries have been implemented to achieve some responsiveness, however not 100% for the new pages.<br>

Another Issue i was facing during development, was that adding a Project via the Admin Front-End caused the Project to be added twice in the DB, <br> 
I believe this was caused by a duplicate Event Listener in my admin.js. Since it now seems to work fine. <br>

127.0.0.1:8080/project/2 wont currently work, as i had to remove a project from the database during development and re add it, causing the id {2} to be left out. <br>
Please use (1, 3, 4, 5, 6) <br>

-----------------
   FEATURES
-----------------

Home Page -> Nothing much to do here <br>
About Page -> Nothing much to do here ( left the same ) <br>
Project Page -> Project can be added to the db and viewed here. -> Additional Project Detail Page upon Link Click ( Templated ) <br>
Contact Page -> Contact Form can be submitted <br>
Greet Page -> Displays a Dynamic Greet message based on the URL Path<br>
Admin Page -> Allows adding, changing and deleting Projects, aswell as view and delete submitted Contact Form Messages. ( Server can keep running ) <br>
   -> Requires Authentication Login. ( Login Details provided in .env ( root of repo) ) (ls -la -> nano .env) )<br>

-----------------
  INSTRUCTIONS
-----------------

1.Install Rust Lang <br>
2.Clone Repository into Folder <br>
3.Cd into Repo<br>
4.type "cargo run"<br>
5.open server on 127.0.0.1:8080<br>
6.Try out the Pages and Routes!<br>
------------------<br>


------------------
   FUTURE GOALS
------------------

Adjust Styling for all Pages, to maintain responsiveness on all screensizes. <br>
Cleanup Code base, remove unneeded or duplicate code entries. <br>
Change Authentication Login storage location. -> Move to DB <br>
Cleanup Navbar and Header Element Positions on Contact Page and About Page. <br>



Credits: <br>

HTML Boilerplate, CSS Boilerplate : Other Website Code from myself. <br>

Image / Gif Sources: <br>

Home Image: kirokaze | Wallpaper Engine <br>
About Image: Pixel Jeff | Behance <br>
Contact Image: Pixel Jeff | Behance <br>
Admin Image: Pixel Jeff | Behance <br>
Project Image: Pixel Jeff | Behance <br>
Greet Image: Pixel Jeff | Behance <br>
