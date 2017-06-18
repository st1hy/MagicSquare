#include <iostream>
#include <thread>
#include <vector>
#include <algorithm>
#include <atomic>
#include <csignal>
#include <chrono>
#include <ctime>
#include <string>
#include <random>
#include <iterator>

#include "main.hpp"

static std::atomic<unsigned long> tries(0);
static std::atomic<bool> run(true);

int main()
{
	signal(SIGINT, signalHandler);  
	
    std::cout << "Magic square!\n";
    auto start = std::chrono::system_clock::now();
    auto cpus = cpu_count();
	auto ttp = std::chrono::system_clock::to_time_t(start);
	std::string timeStr(std::ctime(&ttp));
    std::cout << "Started " << timeStr.substr(0, timeStr.size() -1);
    std::cout << " using " << cpus << " cores\n";
    
    std::vector<std::thread> workers;
    for (int i = 0; i < cpus; i++) {
        workers.push_back(std::thread([]() 
        {
			findSquare();
        }));
    }
    
    std::for_each(workers.begin(), workers.end(), [](auto &t) 
    {
        t.join();
    });
    std::cout << "Tries " << tries << '\n';
    auto end = std::chrono::system_clock::now();
    std::chrono::duration<float> time;
    time = end - start;
    auto timeAsSec = std::chrono::duration_cast<std::chrono::milliseconds>(time);
    auto timeAsFloat = timeAsSec.count() / 1000.0;
    std::printf("Time: %.2f s\n", timeAsFloat);
    std::printf("Tries: %.0f tries / s\n", tries / timeAsFloat);

    return 0;
}

void signalHandler(int signum) {
    std::cout << "\nInterrupted\n";
    run = false;
}

inline int cpu_count() {
    unsigned cpus = std::thread::hardware_concurrency();
    if (cpus < 1) cpus = 1;
    return cpus;
}

void findSquare() {
    std::random_device rd;
	std::mt19937 rand(rd());
    std::uniform_int_distribution<square_int> dist(MIN, MAX);
    square_int square [SIZE][SIZE];
	while (run) {
		tries++;
		populate(square, [&](auto x, auto y) {return dist(rand);});
		powerOf2(square);
		if (is_magic(square)) {
			run = false;
			std::cout << "Found magic " << to_string(square) << '\n';
		}
	}
}

void populate(square_int (&square)[SIZE][SIZE], generator f) {
	for (int x = 0; x < SIZE; x++) {
		for (int y = 0; y < SIZE; y++) {
			square[x][y] = 0;
		}
	}
	for (int x = 0; x < SIZE; x++) {
		for (int y = 0; y < SIZE; y++) {
			square[x][y] = checkGenerate(square, f, x, y);
		}
	}
}

square_int checkGenerate(square_int (&square)[SIZE][SIZE], generator f, int x, int y) {
	while (true) {
		auto newValue= f(x,y);
		if (!contains(square, newValue)) {
			return newValue;
		}
	}
}

bool contains(square_int (&square)[SIZE][SIZE], square_int value) {
	for (int x = 0; x < SIZE; x++) {
		for (int y = 0; y < SIZE; y++) {
			if (square[x][y] == value) {
				return true;
			}
		}
	}
	return false;
}

void powerOf2(square_int (&square)[SIZE][SIZE]) {
	for (int x = 0; x < SIZE; x++) {
		for (int y = 0; y < SIZE; y++) {
			square[x][y] *= square[x][y];
		}
	}
}

bool is_magic(square_int (&square)[SIZE][SIZE]) {
	square_int default_total[] = {0, 0};
	square_int last_total[] = {0, 0};
	square_int diagonals[] = {0, 0};
	for (int x = 0; x < SIZE; x++) {
		square_int total[] = {0, 0};
		for (int y = 0; y < SIZE; y++) {
            total[0] += square[x][y];
            total[1] += square[y][x];
            if (x == y) {
                diagonals[0] += square[x][y];
            }
            if (SIZE - x - 1 == y) {
                diagonals[1] += square[x][y];
            }
		}
		if (!equal(last_total, default_total) && !equal(last_total, total)) {
			return false;
		}
		last_total[0] = total[0];
		last_total[1] = total[1];
	}
    if (last_total[0] == last_total[1] && last_total[0] == diagonals[0] && diagonals[0] == diagonals[1]) {
        return true;
    }
    return false;
}

std::string to_string(square_int (&square)[SIZE][SIZE]) {
	std::string s;
	s += "[";
	for (int x = 0; x < SIZE; x++) {
		s += "[";
		for (int y = 0; y < SIZE; y++) {
			s += std::to_string(square[x][y]) + ",";
		}
		s.pop_back();
		s += "],";
	}
	s.pop_back();
	s+= ']';
	return s;
}

inline bool equal(square_int (&a)[2], square_int (&b)[2]) {
	return a[0] == b[0] && a[1] == b[1];
}
