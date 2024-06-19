library(bgwrkR)

worker <- BackgroundWorker$new("background.R", schedule = "* * * * * *")

#* Echo back the input
#* @param msg The message to echo
#* @get /echo
function(msg = "") {
  list(msg = paste0("The message is: '", msg, "'"))
}
