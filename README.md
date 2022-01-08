# gaedients
this thing does what the name says. it takes a bunch of pride flags and turn them into GIMP gradients (`.ggr` files).  
it uses a custom format powered by 🏳️‍🌈 pride 🏳️‍🌈 .

## the format
`Name : ` Defines a name for the gradient  
`color=#<hex code>` Defines a new color with the name of `color`  
`Ratio: ` Defines a custom ratio for the stripes (usually `1` for no custom ratio)  
Ex: `2:1:2` for the bi flag.  

Example file:
```
Name: [gae] trans flag
pink=#f5a9b8
blue=#6eb6ff
white=#ffffff
Ratio: 1
blue
pink
white
pink
blue
```

## how to use this program
- get `rustc`
- `rustc src/main.rs`
- `./main`  
thats it. no need for cargo since i didn't even knew that was a thing.  
mostly just submit flags you want to add as a pull request and i'll do that for you.  

i'll just print out what its doing and you should be able to figure things out.

## i just want to add a flag
put all your flags (or whatever striped thing you want to make) into `pride/` and submit them as a pull request.  
i'll check all of them and run the program for you. then you can get the `.ggr` files by downloading the zip.
