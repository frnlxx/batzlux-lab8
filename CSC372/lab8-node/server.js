var http = require('http');
	fs = require('fs');

var port = 1337;

function serverStaticFile(res,path,contentType,responseCode){

	if(!responseCode){
		responseCode = 200;
	}

	fs.readFile(__dirname + path, function(err,data){
		if(err){
			res.writeHead(500,  {'Content-Type': 'text/plain'});

			res.end('500 - Internal Error');
		} else{
			res.writeHead(responseCode, {'Content-Type': contentType});

			res.end(data);
		}
	});
}



http.createServer(function(request, res) {

	
	var path = request.url.replace(/\/?(?:\?.*)?$/,'')
			.toLowerCase();

	switch(path){

	case '':
		serverStaticFile(res, '/public/index.html', 'text/html');
		break;

	case '/css/style.css':
		serverStaticFile(res, '/public/css/style.css', 'text/css');
		break;

	//html pages 

	case '/index':
		serverStaticFile(res, '/public/index.html', 'text/html');
		break;

	case '/posts':
		serverStaticFile(res, '/public/posts.html', 'text/html');
		break;

	case '/under-construction':
		serverStaticFile(res, '/public/under-construction.html', 'text/html');
		break;

	case '/contact':
		serverStaticFile(res, '/public/contact.html', 'text/html');
		break;

	//images

	case '/images/404bottom.gif':
		serverStaticFile(res, '/public/images/404bottom.gif', 'image/gif');
		break;

	case '/images/404mid.gif':
		serverStaticFile(res, '/public/images/404mid.gif', 'image/gif');
		break;

	case '/images/blogging.png':
		serverStaticFile(res, '/public/images/blogging.png', 'image/png');
		break;

	case '/images/computer-typing.jpeg':
		serverStaticFile(res, '/public/images/computer-typing.jpeg', 'image/jpeg');
		break;

	case '/images/construction.png':
		serverStaticFile(res, '/public/images/construction.png', 'image/png');
		break;

	case '/images/logo.png':
		serverStaticFile(res, '/public/images/logo.png', 'image/png');
		break;

	case '/images/merch.png':
		serverStaticFile(res, '/public/images/merch.png', 'image/png');
		break;

	case '/images/x.png':
		serverStaticFile(res, '/public/images/x.png', 'image/png');
		break;

	default:
		serverStaticFile(res,'/public/404','text/html', 404);
		break;
	}

}).listen(port);

console.log("Listening...Go to http://localhost:" + port)