/** @type {import('tailwindcss').Config} */
module.exports = {
    content: { 
        files: ["*.html", "./src/**/*.rs"],
    },
    theme: {
        extend: {
            colors: {
                gruvboxDark: {
                  bg: "#282828",
                  bg0: "#282828",
                  bgH: "#1d2021",
                  bgS: "#32302f",
                  bg1: "#3c3836",
                  bg2: "#504945",
                  bg3: "#665c54",
                  bg4: "#7c6f64",

                  fg: "#ebdbb2",
                  fg0: "#fbf1c7",
                  fg1: "#ebdbb2",
                  fg2: "#d5c4a1",
                  fg3: "#bdae93",
                  fg4: "#a89984",

                  red: "#cc241d",
                  red2: "#fb4934",
                  green: "#98971a",
                  green2: "#b8bb26",
                  yellow: "#d79921",
                  yellow2: "#fabd2f",
                  blue: "#458588",
                  blue2: "#83a598",
                  purple: "#b16286",
                  purple2: "#d3869b",
                  aqua: "#689d6a",
                  aqua2: "#8ec07c",
                  orange: "#d65d0e",
                  orange2: "#fe8019",
                  gray: "#a89984",
                  gray2: "#928374"
                },
                gruvbox: {
                  bg: "#fbf1c7",
                  bg0: "#fbf1c7",
                  bgH: "#f9f5d7",
                  bgS: "#f2e5bc",
                  bg1: "#ebdbb2",
                  bg2: "#d5c4a1",
                  bg3: "#bdae93",
                  bg4: "#a89984",

                  fg: "#3c3836",
                  fg0: "#282828",
                  fg1: "#3c3836",
                  fg2: "#504945",
                  fg3: "#665c54",
                  fg4: "#7c6f64",

                  red: "#cc241d",
                  red2: "#9d0006",
                  green: "#98971a",
                  green2: "#79740e",
                  yellow: "#d79921",
                  yellow2: "#b57614",
                  blue: "#458588",
                  blue2: "#076678",
                  purple: "#b16286",
                  purple2: "#8f3f71",
                  aqua: "#689d6a",
                  aqua2: "#427b58",
                  orange: "#d65d0e",
                  orange2: "#af3a03",
                  gray: "#7c6f64",
                  gray2: "#928374"
                },
            },
        },
        fontFamily: {
            'fira-mono': ['"Fira Mono"', 'monospace'],
        },
    },
    plugins: [],
}

