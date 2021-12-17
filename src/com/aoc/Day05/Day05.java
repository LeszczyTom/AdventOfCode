package com.aoc.Day05;

import com.aoc.Day;

import java.util.*;

public class Day05 implements Day {
    public Map<String, List<String>> _map = new HashMap<>();

    @Override
    public String part1(List<String> input) {
        for(String i : input) {
            String[] tmp = i.split(" -> ");
            String[] leftPart = tmp[0].split(",");
            String[] rightPart = tmp[1].split(",");
            populateMap(leftPart, rightPart);
        }
        return getTimesLinesOverlap() + "";
    }

    private int getTimesLinesOverlap() {
        int cpt = 0;
        for(String i : _map.keySet()) {
            Set<String> _set = new HashSet<>(_map.get(i));
            for(String s: _set){
                if(Collections.frequency(_map.get(i),s) >= 2) cpt++;
            }
        }
        return cpt;
    }

    private void populateMap(String[] leftPart, String[] rightPart) {
            if(leftPart[0].equals(rightPart[0])) {
                addVertical(leftPart[0], leftPart[1], rightPart[1]);
            }
            if(leftPart[1].equals(rightPart[1])) {
               addHorizontal(leftPart[1], leftPart[0], rightPart[0]);
            }

    }

    private void addHorizontal(String y, String leftPartX, String rightPartX) {
        Integer[] sorted = sort(Integer.parseInt(leftPartX), Integer.parseInt(rightPartX));

        for(int j = sorted[0]; j <= sorted[1]; j++) {
            if(!_map.containsKey(j + "")) _map.put(j + "", new ArrayList<>());
            _map.get(j + "").add(y);
        }
    }

    private void addVertical(String x, String leftPartY, String rightPartY) {
        if(!_map.containsKey(x)) _map.put(x, new ArrayList<>());
        Integer[] sorted = sort(Integer.parseInt(leftPartY), Integer.parseInt(rightPartY));

        for(int j = sorted[0]; j <= sorted[1]; j++) {
            _map.get(x).add(j + "");
        }
    }

    private Integer[] sort(int i, int j) {
        Integer[] sorted = {i, j};
        if (i > j) {
            int tmp = sorted[0];
            sorted[0] = j;
            sorted[1] = tmp;
        }
        return sorted;
    }

    @Override
    public String part2(List<String> input) {
        for(String i : input) {
            String[] tmp = i.split(" -> ");
            String[] leftPart = tmp[0].split(",");
            String[] rightPart = tmp[1].split(",");
            populateMapWithDiagonalLines(leftPart, rightPart);
        }
        return getTimesLinesOverlap() + "";
    }

    private void populateMapWithDiagonalLines(String[] leftPart, String[] rightPart) {
        populateMap(leftPart, rightPart);

        Integer[] sortedX = sort(Integer.parseInt(leftPart[0]), Integer.parseInt(rightPart[0]));
        Integer[] sortedY = sort(Integer.parseInt(leftPart[1]), Integer.parseInt(rightPart[1]));
        int diffX = sortedX[1] - sortedX[0];
        int diffY = sortedY[1] - sortedY[0];
        if(diffX == diffY) {
            addDiagonals(leftPart, rightPart, diffX);
        }
    }

    private void addDiagonals(String[] leftPart, String[] rightPart, int diff) {
        Integer[] sortedY = sort(Integer.parseInt(leftPart[1]), Integer.parseInt(rightPart[1]));

        if(leftPart[1].equals(sortedY[0] + "")) {
            if(Integer.parseInt(leftPart[0]) > Integer.parseInt(rightPart[0])) {
                addDiagonalRightToLeft(leftPart[0], leftPart[1], diff);
                return;
            }
            addDiagonalLeftToRight(leftPart[0], leftPart[1], diff);
            return;
        }
        if(Integer.parseInt(rightPart[0]) > Integer.parseInt(leftPart[0])) {
            addDiagonalRightToLeft(rightPart[0], rightPart[1], diff);
            return;
        }
        addDiagonalLeftToRight(rightPart[0], rightPart[1], diff);
    }

    private void addDiagonalRightToLeft(String x, String y, int diff) {
        for (int i = 0; i <= diff; i++) {
            int tmpX = Integer.parseInt(x) - i;
            int tmpy = Integer.parseInt(y) + i;
            if (!_map.containsKey(tmpX + "")) _map.put(tmpX + "", new ArrayList<>());
            _map.get(tmpX + "").add(tmpy + "");
        }
    }

    private void addDiagonalLeftToRight(String x, String y, int diff) {
        for (int i = 0; i <= diff; i++) {
            int tmpX = Integer.parseInt(x) + i;
            int tmpY = Integer.parseInt(y) + i;
            if (!_map.containsKey(tmpX + "")) _map.put(tmpX + "", new ArrayList<>());
            _map.get(tmpX + "").add(tmpY + "");
        }
    }
}
