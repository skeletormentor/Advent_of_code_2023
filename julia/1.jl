### A Pluto.jl notebook ###
# v0.19.36

using Markdown
using InteractiveUtils

# ╔═╡ 05a70ae0-a672-11ee-087f-ad829e9a2acc
lines = readlines(raw"C:\Users\vanarinova\Advent_of_code_2023\input\input1.txt")

# ╔═╡ be006dcc-ed7e-4295-afb7-aea32a3d330a
function calibrate(lines::Vector{String})::Int
	s = 0	
	for line in lines
		nums = [char for char in line if isnumeric(char)]
		num = nums[1] * nums[end]
		s += parse(Int, num)
	end
	return s
end

# ╔═╡ a2e34a54-4349-4d8b-87bd-3e1ca44b500c
calibrate(lines)

# ╔═╡ 43748746-73d4-40b9-b7a1-f8c306f5a51d
convert = split("one two three four five six seven eight nine")

# ╔═╡ dd4f1e92-7193-4860-80de-0f1a28f1a1e4
function calibrate2(lines)
	s = 0
	for line in lines
		nums = []
		linelength = length(line)
		for i in 1:linelength
			for (idx, text) in enumerate(convert)
				if startswith(line[i:linelength], text)
					push!(nums, string(idx))
				end
			end
			if isnumeric(line[i])
				push!(nums, line[i])
			end
		end
		num = nums[1] * nums[end]
		s += parse(Int, num)
	end
	return s
end

# ╔═╡ df020fba-9183-40f9-a724-921a1dc06187
calibrate2(lines)

# ╔═╡ Cell order:
# ╠═05a70ae0-a672-11ee-087f-ad829e9a2acc
# ╠═a2e34a54-4349-4d8b-87bd-3e1ca44b500c
# ╠═df020fba-9183-40f9-a724-921a1dc06187
# ╠═be006dcc-ed7e-4295-afb7-aea32a3d330a
# ╠═43748746-73d4-40b9-b7a1-f8c306f5a51d
# ╠═dd4f1e92-7193-4860-80de-0f1a28f1a1e4
