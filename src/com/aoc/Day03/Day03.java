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
        return input.get(0);
    }
}




















