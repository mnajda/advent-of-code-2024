#include <algorithm>
#include <filesystem>
#include <fstream>
#include <iostream>
#include <ranges>
#include <stdexcept>
#include <string>
#include <vector>
#include <unordered_set>
#include <unordered_map>

using Point = std::pair<int, int>;

namespace std
{

template <>
struct hash<Point>
{
    std::size_t operator()(const Point& point) const
    {
        return std::hash<int>{}(point.first) ^ std::hash<int>{}(point.second);
    }
};

}

std::vector<std::string> LoadFile(const std::filesystem::path& filepath)
{
    auto result = std::vector<std::string>{};

    auto file = std::ifstream{filepath};
    auto input = std::string{};
    while (std::getline(file, input))
    {
        result.push_back(std::move(input));
    }

    return result;
}

std::unordered_map<char, std::vector<Point>> FindAntennas(const std::vector<std::string>& input)
{
    auto antennas = std::unordered_map<char, std::vector<Point>>{};

    for (const auto& [y, col] : std::views::enumerate(input))
    {
        for (const auto& [x, cell] : std::views::enumerate(col))
        {
            if (cell != '.')
            {
                antennas[cell].push_back(Point{y, x});
            }
        }
    }

    return antennas;
}

bool IsInBounds(const Point& point, const int size)
{
    return point.first >= 0 && point.first < size && point.second >= 0 && point.second < size;
}

std::vector<Point> GetAntinodesFor(const Point& a, const Point& b, const int size)
{
    auto result = std::vector<Point>{};

    const auto diff = Point{a.first - b.first, a.second - b.second};

    const auto c = Point{a.first + diff.first, a.second + diff.second};
    const auto d = Point{b.first - diff.first, b.second - diff.second};

    if (IsInBounds(c, size))
    {
        result.push_back(c);
    }

    if (IsInBounds(d, size))
    {
        result.push_back(d);
    }

    return result;
}

void Part1(const std::vector<std::string>& input)
{
    auto antinodes = std::unordered_set<Point>{};
    const auto antennas = FindAntennas(input);

    for (const auto& [key, value] : antennas)
    {
        for (const auto& points : std::views::cartesian_product(value, value | std::views::drop(1)))
        {
            const auto& [a, b] = points;

            if (a == b)
            {
                continue;
            }

            const auto nodes = GetAntinodesFor(a, b, input.size());
            antinodes.insert(nodes.begin(), nodes.end());
        }
    }

    std::cout << "Part 1 solution is " << antinodes.size() << '\n';
}

void Part2(const std::vector<std::string>& input)
{
    auto antinodes = std::unordered_set<Point>{};
    const auto antennas = FindAntennas(input);

    for (const auto& [key, value] : antennas)
    {
        for (const auto& points : std::views::cartesian_product(value, value | std::views::drop(1)))
        {
            auto [a, b] = points;

            if (a == b)
            {
                continue;
            }

            const auto diff = Point{a.first - b.first, a.second - b.second};
            auto antinode = b;

            while (IsInBounds(antinode, input.size()))
            {
                antinodes.insert(antinode);
                antinode = Point{antinode.first + diff.first, antinode.second + diff.second};
            }

            antinode = a;
            while (IsInBounds(antinode, input.size()))
            {
                antinodes.insert(antinode);
                antinode = Point{antinode.first - diff.first, antinode.second - diff.second};
            }
        }
    }

    std::cout << "Part 2 solution is " << antinodes.size() << '\n';
}

int main(int argc, char** argv)
{
    if (argc < 2)
    {
        throw std::runtime_error("Provide filepath");
    }

    const auto input = LoadFile(argv[1]);

    Part1(input);
    Part2(input);
}
