#Pixel Portfolio
Backend Rust ( Actix Web) 

----------------
   GENERAL
----------------

DISCLAIMER! 
THIS CODE IS NOT YET FINISHED AND WILL SHOW FLAWS AS SUCH!<br>
THERE MIGHT BE CODE THAT DOES NOTHING AT ALL, LEFT INSIDE!<br>

This is a basic Pixel Portfolio Page <br>
Each Page (Home, About,Projects, Contact and Greet) has its individual styling sheet.<br> 
Media Queries have been implemented to achieve a very "basic" responsive layout. <br>
-> They Currently do not support the smallest nor biggest screen sizes. <br>
-> This will be fixed in the following version. <br>
Glass Morphism Styling was generated from css.glass <br>
Project Cards, Descriptions and Images have been implemented.<br>
A Parallax Scrolling animation has been implemented for the Project page. <br>

<br>
<br>
<br>  

The Backend was built with the Rust Actix Web Framework <br>
It currently handles 4 static routes and a single dynamic route. <br>
These Routes can be accessed via 127.0.0.1:8080/ <br>
                                 127.0.0.1:8080/about<br>
                                 127.0.0.1:8080/contact<br>
                                 127.0.0.1:8080/projects<br>
                  Dynamic Route  127.0.0.1:8080/greet/Momo  <br>
                  <br>
The Dynamic Route Displays a greet message based on the URL Path chosen. <br>


-----------------
  CURRENT ISSUES
-----------------

For some reason, during my testing, the CSS for the Project Page is being rendered correctly when run with a live server extension ( running on 127.0.0.1:5500) from vs code. <br>
However on my system, when loading the project via the actix web server on port 127.0.0.1:8080 the Page is being rendered a lot differently and faulty. <br>
I currently do not know why this is happening. <br>


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
To check new CSS for Project Page, open in VS Code and run with Live Server Extension. 


Credits: <br>

HTML Boilerplate, CSS Boilerplate : Other Website Code from myself. <br>

Image / Gif Sources: <br>

Home Image: kirokaze | Wallpaper Engine <br>
About Image: Pixel Jeff | Behance <br>
Contact Image: Pixel Jeff | Behance <br>
Project Image: Pixel Jeff | Behance <br>
Greet Image: Pixel Jeff | Behance <br>
