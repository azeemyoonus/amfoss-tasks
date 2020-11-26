var message = "Hi"; //spam message 
var count = 15 ; //number of times to send
var i = 0 ;
var spam = setInterval(function(){
	document.getElementsByClassName('composer_rich_textarea')[0].innerHTML = message;
	$('.im_submit').trigger('mousedown');	
	i++;
	if( i  == count )
	clearInterval(spam);
} ,1500 ) ;
