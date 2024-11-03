using System;
using System.Numerics;
using System.Collections.Generic;

[Uuid("7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
public class Beacon : Script
{
    public override void OnUpdate()
    {
        Vector3 pos = this.Node.GlobalPosition;

        Plugin.Get<Game>("Game").Beacons.Add(pos);

        Console.WriteLine($"Beacon registered: {this.Node}");

        this.Node.Destroy();
    }
}