using System;
using System.Numerics;
using System.Collections.Generic;

[Uuid("c69ae5fa-de26-4ee5-b70c-113df285f6e2")]
public class GuardChief : Script
{
    public Prefab GuardPrefab { get; set; }
    public int InitialCount { get; set; }
    private bool Initialized { get; set; }
    private bool FrameSkippedForBeacons { get; set; }

    public override void OnUpdate(float dt)
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
                List<Vector3> beacons = Plugin.Get<Game>("Game").Beacons;
                if (beacons.Count > 0)
                {
                    Vector3 position = beacons[new Random().Next(beacons.Count)];
                    Quaternion orientation = Quaternion.CreateFromAxisAngle(Vector3.UnitY, (float)(new Random().NextDouble() * 2 * Math.PI));

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
