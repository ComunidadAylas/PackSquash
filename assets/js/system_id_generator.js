var generateIdButton = document.getElementById("generate_id");
var copyIdButton = document.getElementById("copy_id");
var systemIdBox = document.getElementById("system_id");
var idGeneratedOnce = false;

function uuidv4() {
	var crypto = window.crypto || window.msCrypto;
	return ([1e7]+-1e3+-4e3+-8e3+-1e11).replace(/[018]/g, function(c) {
		return (c ^ crypto.getRandomValues(new Uint8Array(1))[0] & 15 >> c / 4).toString(16);
	});
}

function generateSystemId() {
	systemIdBox.innerText = uuidv4();
	idGeneratedOnce = true;
}

function copySystemId() {
	if (idGeneratedOnce) {
		var selection = getSelection();
		var range = document.createRange();
		range.selectNodeContents(systemIdBox);
		selection.removeAllRanges();
		selection.addRange(range);

		if (document.execCommand("copy", false)) {
			alert("The system identifier was copied to the clipboard.");
		}
	}
}

generateIdButton.onclick = generateSystemId;
generateIdButton.onkeyup = function(evt) {
	evt.preventDefault();
	if (evt.key == "Enter") {
		evt.target.onclick();
	}
};
generateIdButton.onmouseleave = function(evt) {
	evt.target.blur();
};

copyIdButton.onclick = copySystemId;
copyIdButton.onkeyup = function(evt) {
	evt.preventDefault();
	if (evt.key == "Enter") {
		evt.target.onclick();
	}
};
copyIdButton.onmouseleave = function(evt) {
	evt.target.blur();
};

var hiddenElements = document.querySelectorAll(".hide_no_js");
for (var i = 0; i < hiddenElements.length; ++i) {
	hiddenElements[i].classList.remove("hide_no_js");
}
