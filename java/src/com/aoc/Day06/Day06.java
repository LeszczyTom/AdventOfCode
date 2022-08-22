package com.aoc.Day06;

import com.aoc.Day;

import java.util.*;

public class Day06 implements Day {
    int cpt = 0;
    @Override
    public String part1(List<String> input) {
        String[] initialState = input.get(0).split(",");
        int days = 80;
        for(String n : initialState) {
            fish(Integer.parseInt(n), days);
        }
        return cpt + "";
    }

    private void fish(int nb, int days) {
        cpt++;
        for(int i = 0; i <= days; i++) {
            nb--;
            if(nb == -1 && days - 1 - i >= 0) {
                nb = 6;
                fish(8, (days - i - 1));
            }
        }
    }

    Map<Integer, Long> fishs;

    @Override
    public String part2(List<String> input) {
        String[] listInput = input.get(0).split(",");
        List<Integer> initialState = new ArrayList<>();
        for(String i : listInput) initialState.add(Integer.parseInt(i));

        int days = 256;
        fishs = new HashMap<>();
        for(int i = 0; i < 9; i++) fishs.put(i, 0L);

        for(Integer i : initialState) {
            fishs.put(i, fishs.get(i) + 1);
        }
        while(days != 0) {
            day();
            days--;
        }

        Long res = 0L;
        for(int i : fishs.keySet()) {
            res += fishs.get(i);
        }
        return res + "";
    }

    private void day() {
        Long tmp = fishs.get(0);
        fishs.put(0, fishs.get(1));
        fishs.put(1, fishs.get(2));
        fishs.put(2, fishs.get(3));
        fishs.put(3, fishs.get(4));
        fishs.put(4, fishs.get(5));
        fishs.put(5, fishs.get(6));
        fishs.put(6, fishs.get(7) + tmp);
        fishs.put(7, fishs.get(8));
        fishs.put(8, tmp);
        //System.out.println(fishs);
    }
}
