using System;
using System.Collections.Generic;
using System.Drawing;
using FyroxLite;

public class Game : GlobalScript
{
    public Node Player;
    public List<Vector3> Beacons;
    private int Frags;
    private int Wounds;
    private Text Hud;

    protected override void OnGlobalInit()
    {
        Scene.LoadAsync("data/scene.rgs");

        Hud = Text.New(new TextBuilder
        {
            FontSize = 40,
            Foreground = new Brush
            {
                SolidColor = Color.Black
            }
        });

        Beacons = new List<Vector3>();
    }

    protected override void OnGlobalUpdate()
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