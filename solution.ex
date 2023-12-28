# The goal is to count the frequency of consecutives strings in a row.

str = "nBalooonnn"
target = String.split(str, "", trim: true)

 Enum.reduce(0..(String.length(str) - 1), %{}, fn idx, acc ->
  current_str = Enum.at(target, idx)
  next_str = Enum.at(target, idx + 1)
  value = Map.get(acc, current_str)

  if value do
    if current_str == next_str do
      cond do
        value > 1 ->
          Map.put(acc, current_str, value + 1)
        value < 1 ->
          Map.put(acc, current_str, value + 2)
        true ->
          acc
      end
    else
      acc
    end
  else
    if current_str == next_str do
      Map.put(acc, current_str, 2)
    else
      Map.put(acc, current_str, 0)
    end
  end
end)
|> IO.inspect()

# %{"B" => 0, "a" => 0, "l" => 0, "n" => 3, "o" => 3}
