Our project consists of two parts: the client part and the server part. 

The server part can be run directly on the terminal of its rust code. It opens up the terminal as the server, and automatically deals with communication with every client without any manual inputs.

The client part, meanwhile, can't be run on its own. Considering the need to generate multiple clients on one computer, we created "client.exe" as an executable starter for a client. The client can only function when there is a server, and relies heavily on manually inputing commands.

There will be a block where you can enter commands after opening y=up "client.exe". There are several commands available. Remember that to make most codes function, at least two clients need to be generated. Here are the commands available:

"code": Binds a code with the address of the client. Other clients can use the code bound to send messages.
Format: code user_code
Example: code Luden

"exit": Quits the server and exits the executable. The server will know that you have exited. 
Format / Example: exit

"sendmsg": Sends a private message to the intended user. Uses the server as a medium. Will notify the sender client if the message fails to reach the receiver client.
Format: send user_code "message"
Example: send Luden "How are you?"

"broadcast": Sends a broadcasted message to all other online users. If there are no other clients connected, the server will notify the sender client that broadcasting will not work.
Format: broadcast "message"
Example: broadcast "Howdy, world!"

"getusers": Gets a list of online client codes and their corresponding addresses. Will show all online clients, and mark the one that corresponds to the client itself.
Format / Example: getusers

"reply": Replies to the last user who has sent you a message. Can reply to anonymous clients.
Format: reply "message"
Example: reply "Fine, thank you!"
