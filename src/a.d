import std;

void main()
{
    auto s = File("../content.txt", "r").byLineCopy().array.fold!((acc, x) => acc ~ x)("");
    //s.split("</td><td").each!writeln;
    auto r = "class=\"name\">(\\w+)<";
    r = "<img src=\"/blackwhite/pokemon/(\\d+).png\" class=\"wildsprite\"";

    matchAll(s, r).each!writeln();
    //matchAll(s, "Standard Walking").writeln;
    //matchAll(s, "Interaction").writeln;
    //<img src="/blackwhite/pokemon/551.png" class="wildsprite"
    each!writeln("Normal Fire Water Electric Grass Ice Fighting Poison Ground Flying Psychic Bug Rock Ghost Dragon Dark Steel Fairy ".split());
}