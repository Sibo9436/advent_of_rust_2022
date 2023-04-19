# Idea
I could use a fake density distribution to detect
whether a cube is inside or outside of the droplet
Something like a distance function or smth
Another option could be to just iterate through all adjacent 
empty blocks to calculate the surface area of a pocket and if I go out of some the droplets
bound I know it's just a convex shape and not a pocket
If it doesn't it will just end, I could then use a set of these
empty blocks to calculate their surface area 
Un'idea interessante sarebbe designare un cubo esterno come limite
di zone d'aria esterne e da lì ricalcolare la superficie avvicinandoci
Oppure usarle in una mappa per identificare quali bolle sono all'interno e quali
sono concavità