### A Pluto.jl notebook ###
# v0.19.36

using Markdown
using InteractiveUtils

# ╔═╡ 3e5c0dcd-8e3e-4c19-8d68-73a63a6496dd
lines = readlines(raw"C:\Users\vanarinova\Advent_of_code_2023\input\input2.txt")
# lines = split("""Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
# Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
# Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
# Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
# Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green""", '\n')

# ╔═╡ 9a6a8ae1-1c86-4d6d-b557-33ac8e9ed4e3
function parsed(lines)
	gamelist = []
	for line in lines
		bags = []
		_, texts = strip.(split(line, ":"))
		texts = split(texts, ";")
		for text in texts
			picks = eachmatch(r"(\d+) (\w+)", text)
			pick = Dict(p[2] => parse(Int, p[1]) for p in picks)
			push!(bags, pick)
		end
		push!(gamelist, bags)
	end
	return gamelist
end

# ╔═╡ d8a80264-48bd-4eed-819d-09fbf70ee515
function validate(gamelist)
	s = 0
	max_values = Dict("red" => 12, "green" => 13, "blue" => 14)
	for (i, game) in enumerate(gamelist)
		isvalid = true
		for grab in game
			for (color, amount) in grab
				if amount > max_values[color]
					isvalid = false
				end
			end
		end
		if isvalid
			s += i
		end
	end
	return s
end

# ╔═╡ 8fc8bcc5-f864-463e-9f49-345f67042df1
function get_max(gamelist)
    s = 0
    for game in gamelist
        max_values = Dict()
        for grab in game
            for (color, amount) in grab
                if color in keys(max_values)
                    if amount >= max_values[color]
                        max_values[color] = amount
					end
                else
                    max_values[color] = amount
				end
			end
		end
        s += prod(values(max_values))
	end
    return s
end

# ╔═╡ 4090771f-d8e6-4577-bb8a-5917c7de3c15
validate(parsed(lines))

# ╔═╡ 09189cc3-c455-4ea7-91db-1836e5296aa9
get_max(parsed(lines))

# ╔═╡ Cell order:
# ╠═3e5c0dcd-8e3e-4c19-8d68-73a63a6496dd
# ╠═9a6a8ae1-1c86-4d6d-b557-33ac8e9ed4e3
# ╠═d8a80264-48bd-4eed-819d-09fbf70ee515
# ╠═8fc8bcc5-f864-463e-9f49-345f67042df1
# ╠═4090771f-d8e6-4577-bb8a-5917c7de3c15
# ╠═09189cc3-c455-4ea7-91db-1836e5296aa9
