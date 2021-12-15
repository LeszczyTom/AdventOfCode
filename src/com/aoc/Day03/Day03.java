package com.aoc.Day03;

import com.aoc.Day;

import java.util.*;

public class Day03 implements Day {
    @Override
    public String part1(List<String> input) {
        int longueur = input.size();
        int[] somme = new int[input.get(0).length()];
        Arrays.fill(somme, 0);

        StringBuilder epsilon = new StringBuilder();
        StringBuilder gamma = new StringBuilder();
        for(String i : input) {
            for(int j = 0; j < somme.length; j++) {
                if(i.charAt(j) == '1') somme[j]++;
            }
        }
        for (int j : somme) {
            if (j > longueur / 2) {
                epsilon.append('1');
                gamma.append('0');
            } else {
                epsilon.append('0');
                gamma.append('1');
            }
        }
        return Integer.parseInt(epsilon.toString(),2) * Integer.parseInt(gamma.toString(),2) + "";
    }

    @Override
    public String part2(List<String> input) {
        Map<Integer, String> map = new HashMap<>();
        List<Integer> potentialsOxygenGeneratorRating = new ArrayList<>();
        List<Integer> potentialsCO2ScrubberRating = new ArrayList<>();

        for(int i = 0; i < input.size(); i++ ) {
            map.put(i, input.get(i));
            potentialsCO2ScrubberRating.add(i);
            potentialsOxygenGeneratorRating.add(i);
        }

        for(int ind = 0, length = map.size(); ind < length; ind++) {
            if(potentialsOxygenGeneratorRating.size() != 1) {
                int sum = 0;
                char majority = '0';
                for (int i : potentialsOxygenGeneratorRating) {
                    if (map.get(i).charAt(ind) == '1') {
                        sum++;
                    }
                }
                if (sum >= (float)potentialsOxygenGeneratorRating.size() / 2) majority = '1';

                List<Integer> tmp = new ArrayList<>();
                for (int i : potentialsOxygenGeneratorRating) {
                    if (map.get(i).charAt(ind) == majority) tmp.add(i);
                }
                potentialsOxygenGeneratorRating = tmp;
            }
        }

        for(int ind = 0, length = map.size(); ind < length; ind++) {
            if(potentialsCO2ScrubberRating.size() > 1) {
                int sum = 0;
                char minority = '1';
                for (int i : potentialsCO2ScrubberRating) {
                    if (map.get(i).charAt(ind) == '1') {
                        sum++;
                    }
                }
                if (sum >= (float)potentialsCO2ScrubberRating.size() / 2) minority = '0';

                List<Integer> tmp = new ArrayList<>();
                for (int i : potentialsCO2ScrubberRating) {
                    if (map.get(i).charAt(ind) == minority) tmp.add(i);
                }
                potentialsCO2ScrubberRating = tmp;
            }
        }
        return Integer.parseInt(map.get(potentialsCO2ScrubberRating.get(0)), 2) * Integer.parseInt(map.get(potentialsOxygenGeneratorRating.get(0)), 2) + "";
    }
}




















