using System;
using System.Numerics;
using System.Collections.Generic;
using FyroxLite;

[Uuid("7c259fd2-fdb9-453b-a7ef-19cdd85428cc")]
public class Beacon : NodeScript
{
    public void OnUpdate(float dt)
    {
        Vector3 pos = this.Node.GlobalPosition;

        Plugin.Get<Game>().Beacons.Add(pos);

        Console.WriteLine($"Beacon registered: {this.Node}");

        this.Node.Destroy();
    }
}