using System;
using System.Collections.Generic;
using System.Drawing;
using FyroxLite;

[Uuid("c69ae5fa-de26-4ee5-b70c-113df285f6e2")]
public class GuardChief : NodeScript
{
    private Prefab GuardPrefab;
    private int InitialCount;
    
    [HideInInspector]
    [Transient]
    private bool Initialized;
    
    [HideInInspector]
    [Transient]
    private bool FrameSkippedForBeacons;

    public void OnUpdate(float dt)
    {
        if (!FrameSkippedForBeacons)
        {
            FrameSkippedForBeacons = true;
            return;
        }

        if (!Initialized)
        {
            Initialized = true;
            for (int i = 1; i <= InitialCount; i++)
            {
                List<Vector3> beacons = Plugin.Get<Game>().Beacons;
                if (beacons.Count > 0)
                {
                    Vector3 position = beacons[new Random().Next(beacons.Count)];
                    var angle = (float)(new Random().NextDouble() * 2 * Math.PI);
                    Quaternion orientation = Quaternion.FromEuler(Vector3.Up * angle);

                    Node guard = GuardPrefab.InstantiateAt(position, orientation);
                    guard.FindScript<Guard>().Init(i);

                    Log.Info($"Guard spawned at {position}");
                }
                else
                {
                    Log.Err("Cannot spawn guards: no beacons found");
                }
            }
        }
    }
}