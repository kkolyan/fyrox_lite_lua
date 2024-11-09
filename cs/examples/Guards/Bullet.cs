using System;
using System.Collections.Generic;
using FyroxLite;

[Uuid("12371d19-9f1a-4286-8486-add4ebaadaec")]
public class Bullet : NodeScript
{
    private Vector3 velocity;
    private float remaining_sec;
    private Node author_collider;
    private float fraction;

    public struct BulletSeed
    {
        public Prefab Prefab;
        public Vector3 Origin;
        public Vector3 Direction;
        public float InitialVelocity;
        public Node AuthorCollider;
        public float Range;
        public int Fraction;
    }

    public static void Spawn(BulletSeed seed)
    {
        var orientation = Basis.LookingAt(seed.Direction, Vector3.Up).GetRotationQuaternion();
        var bullet = seed.Prefab.InstantiateAt(seed.Origin, orientation);
        var script = bullet.FindScript<Bullet>();
        script.velocity = seed.Direction.Normalized() * seed.InitialVelocity;
        script.remaining_sec = seed.Range / seed.InitialVelocity;
        script.author_collider = seed.AuthorCollider;
        script.fraction = seed.Fraction;
    }

    protected override void OnUpdate(float deltaTime)
    {
        remaining_sec -= deltaTime;
        if (remaining_sec <= 0.0f)
        {
            Node.Destroy();
            return;
        }

        List<Intersection> results = Physics.CastRay(new RayCastOptions
        {
            RayOrigin = Node.LocalPosition,
            RayDirection = velocity.Normalized(),
            MaxLen = velocity.Length() * deltaTime,
            SortResults = true
        });

        foreach (var hit in results)
        {
            if (hit.Collider != author_collider)
            {
                // scene from the Lua version of game is used, and Lua stores any number as f32
                var fraction = (int)this.fraction;

                hit.Collider.SendHierarchical(RoutingStrategy.Up, new BulletHitMessage { Fraction = fraction });
                Node.Destroy();
                return;
            }
        }

        Node.LocalPosition += velocity * deltaTime;
    }
}