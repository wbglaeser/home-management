//***************
// FORM MAGIC
//***************
function suggest_input(param0, availableTutorials) {
    $(param0).autocomplete({
        source: availableTutorials
    });
}
//**********************
// Submit Form
//**********************
function form_submit() {
    document.getElementById("search_form").submit();
}

$("#test").hover(function(){
    $('.flyout').show();
},function(){
    $('.flyout').hide();
});
