using System;
using System.Numerics;
using System.Collections.Generic;
using System.Drawing;
using FyroxLite;

public class Game : GlobalScript
{
    public Node Player { get; set; }
    public List<Vector3> Beacons { get; set; }
    public int Frags { get; private set; }
    public int Wounds { get; private set; }
    public Text Hud { get; set; }

    public void Init(string scenePath = "data/scene.rgs")
    {
        Scene.LoadAsync(scenePath);

        Hud = new Text
        {
            FontSize = 40,
            Foreground = Brush.Solid(Color.Black)
        };

        Beacons = new List<Vector3>();
    }

    public void OnGlobalUpdate()
    {
        Hud.TextAsync = $"Wounds: {Wounds}\nKilled Guards: {Frags}";
    }

    public void IncFrags()
    {
        Frags += 1;
    }

    public void IncWounds()
    {
        Wounds += 1;
    }
}