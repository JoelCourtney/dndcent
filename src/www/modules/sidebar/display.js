registerDisplayCallback(function (character) {
    pasteFields(["name", "race_name"], character);
    let classes = character.class_names[0];
    for (let i = 1; i < character.class_names.length; i++) {
        classes += " | " + character.class_names[i];
    }
    document.getElementById("field-class_names").innerHTML = classes;
});