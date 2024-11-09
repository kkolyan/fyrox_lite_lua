using System;
using System.Collections.Generic;
using System.Drawing;
using FyroxLite;

public class Game : GlobalScript
{
    public Node player;
    [Transient] public List<Vector3> beacons;
    private int frags;
    private int wounds;
    private Text hud;

    protected override void OnGlobalInit(string? initialSceneOverride)
    {
        Scene.LoadAsync(initialSceneOverride ?? "data/scene.rgs");

        hud = Text.New(new TextBuilder
        {
            FontSize = 40,
            Foreground = new Brush
            {
                SolidColor = Color.Black
            }
        });

        beacons = new List<Vector3>();
    }

    protected override void OnGlobalUpdate()
    {
        hud.TextAsync = $"Wounds: {wounds}\nKilled Guards: {frags}";
    }

    public void IncFrags()
    {
        frags += 1;
    }

    public void IncWounds()
    {
        wounds += 1;
    }
}