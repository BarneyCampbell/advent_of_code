
function part1(maps)
    for map in maps
        println("Map:")
        for line in map
            println(line)
        end
    end
end

function main()
    open("data/day13small.txt") do file
        lines = readlines(file)
        maps = []
        map = []

        for line in lines
            if line == "" 
                push!(maps, copy(map))
                map = []
            else
                push!(map, line)
            end
        end

        part1(maps)
    end
end

main()
