package me.ikate.main;

import me.ikate.greeter.Greeter;

public class Main {

    public static void main(String[] args) {
        System.out.println("Bazel packaged");
        Greeter greeter = new Greeter();
        System.out.println(greeter.greet());
    }

}