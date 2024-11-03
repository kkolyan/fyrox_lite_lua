using System;
using System.Numerics;
using System.Collections.Generic;
using FyroxLite;

[Uuid("12371d19-9f1a-4286-8486-add4ebaadaec")]
public class Bullet : NodeScript
{
    public Vector3 Velocity { get; set; }
    public float RemainingSeconds { get; set; }
    public Node AuthorCollider { get; set; }
    public float Fraction { get; set; }

    public static readonly string HitMessage = "BulletHit";

    public class BulletSeed
    {
        public Prefab Prefab { get; set; }
        public Vector3 Origin { get; set; }
        public Vector3 Direction { get; set; }
        public float InitialVelocity { get; set; }
        public Node AuthorCollider { get; set; }
        public float Range { get; set; }
        public float Fraction { get; set; }
    }

    public static void Spawn(BulletSeed seed)
    {
        Quaternion orientation = Quaternion.FaceTowards(seed.Direction, Vector3.UnitY);
        Node bullet = seed.Prefab.InstantiateAt(seed.Origin, orientation);
        Bullet script = bullet.FindScript<Bullet>();
        script.Velocity = Vector3.Normalize(seed.Direction) * seed.InitialVelocity;
        script.RemainingSeconds = seed.range / seed.initial_velocity
        script.AuthorCollider = seed.AuthorCollider;
        script.Fraction = seed.Fraction;
    }

    public override void OnUpdate(float deltaTime)
    {
        RemainingSeconds -= deltaTime;
        if (RemainingSeconds <= 0.0f)
        {
            Node.Destroy();
            return;
        }

        Vector3 newPos = Node.LocalPosition + Velocity * deltaTime;

        RayCastOptions opts = new RayCastOptions
        {
            RayOrigin = Node.LocalPosition,
            RayDirection = Vector3.Normalize(Velocity),
            MaxLength = Velocity.Length() * deltaTime,
            SortResults = true
        };

        List<Intersection> results = Physics.CastRay(opts);

        foreach (var hit in results)
        {
            if (hit.Collider != AuthorCollider)
            {
                hit.Collider.SendHierarchical(RoutingStrategy.Up, new { type = HitMessage, fraction = Fraction });
                Node.Destroy();
                return;
            }
        }

        Node.LocalPosition = newPos;
    }
}