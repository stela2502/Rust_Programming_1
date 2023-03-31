scriptF = file.path( "target","release","kmeans")
dataF   = file.path( "testData","CellexalVR_TestData_tsne.csv" )
ofile  = file.path( "testData","Clustering.txt" )

cmd =  paste( scriptF, "-d", dataF, "-s ',' -c 19 -m 10 -o", ofile)
print ( cmd) 
system( cmd )


data = read.delim( dataF, sep=",", row.names=1)
grouping = read.delim( ofile, sep=",", header=F, row.names=1)
centroids = read.delim( file.path("testData","centroid.csv" ), sep=",", header=F )

startTime <- Sys.time()
km = kmeans( data, 19)
endTime <- Sys.time()
print(paste("R kmenas algo on this data:" ,round( endTime - startTime, digits = 4), "sec"))

library(rgl)



plot3d( data, col=rainbow(19)[grouping[,1]+1] )
points3d( centroids, col=rainbow(19), size=40, alpha = 0.6)


data = read.delim('testData/PCS_data.csv', sep=",", row.names=1)
startTime <- Sys.time()
km = kmeans( data, 42)
endTime <- Sys.time()
print(paste("R kmens algo on the larger data:" ,round( endTime - startTime, digits = 4), "sec"))
