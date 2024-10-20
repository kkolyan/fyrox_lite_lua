using System;
using System.Numerics;

[Uuid("c5671d19-9f1a-4286-8486-add4ebaadaec")]
public class Player : Script
{
    public float Sensitivity { get; set; }
    public Node Camera { get; set; }
    public float Power { get; set; }
    public Prefab Bullet { get; set; }
    public float InitialBulletVelocity { get; set; }
    public float ShootingRange { get; set; }
    public float ReloadDelaySec { get; set; }

    private float reloadSec = 0;
    private bool published = false;
    private Node collider;
    private static float aimY = 0;
    private static bool forward = false;
    private static bool back = false;
    private static bool left = false;
    private static bool right = false;
    private static bool fire = false;

    private const int FRACTION_PLAYER = 0;

    public void Turn(float x)
    {
        Quaternion rotDelta = Quaternion.CreateFromAxisAngle(Vector3.UnitY, Sensitivity * x);
        node.LocalRotation = node.LocalRotation * rotDelta;
    }

    public void Aim(float y)
    {
        aimY += y * Sensitivity;
        aimY = Math.Clamp(aimY, -(float)Math.PI / 2.0f, (float)Math.PI / 2.0f);

        Camera.LocalRotation = Quaternion.CreateFromAxisAngle(Vector3.UnitX, aimY);
    }

    public void Fire()
    {
        Vector3 cameraPos = Camera.GlobalPosition;
        Quaternion bulletOrientation = Camera.GlobalRotation;

        Bullet.Spawn(new BulletSeed
        {
            Prefab = Bullet,
            Origin = cameraPos,
            Direction = Vector3.Transform(Vector3.UnitZ, bulletOrientation),
            InitialVelocity = InitialBulletVelocity,
            AuthorCollider = collider,
            Range = ShootingRange,
            Fraction = FRACTION_PLAYER
        });
    }

    public override void OnInit()
    {
        Window.CursorGrab = CursorGrabMode.Confined;
        collider = node.FindColliderInChildren() ?? throw new Exception("Player collider missing");
    }

    public override void OnStart()
    {
        node.SubscribeTo();
    }

    public override void OnMessage(Message message)
    {
        if (message.Type == Bullet.HitMessage && message.Fraction != FRACTION_PLAYER)
        {
            Plugin.Get<Game>("Game").IncWounds();
            Console.WriteLine("Player wounded!");
        }
    }

    public override void OnUpdate(float dt)
    {
        if (reloadSec > 0.0f)
        {
            reloadSec -= dt;
        }

        if (!published)
        {
            published = true;
            Plugin.Get<Game>("Game").Player = node;
        }

        if (fire && reloadSec <= 0.0f)
        {
            reloadSec = ReloadDelaySec;
            Fire();
        }

        Vector3 moveDelta = Vector3.Zero;
        if (forward) moveDelta.Z += 1.0f;
        if (back) moveDelta.Z -= 1.0f;
        if (left) moveDelta.X += 1.0f;
        if (right) moveDelta.X -= 1.0f;

        if (moveDelta.Length() > 0.001f) moveDelta = Vector3.Normalize(moveDelta);

        moveDelta = Vector3.Transform(moveDelta, node.LocalRotation);
        Vector3 force = moveDelta * Power;
        node.AsRigidBody().ApplyForce(force);
    }

    public override void OnOsEvent(Event evt)
    {
        if (evt is WindowEvent we)
        {
            if (we.Event is KeyboardInput ki)
            {
                bool value = ki.Event.State == InputState.Pressed;
                switch (ki.Event.PhysicalKey.Code)
                {
                    case KeyCode.KeyW: forward = value; break;
                    case KeyCode.KeyS: back = value; break;
                    case KeyCode.KeyA: left = value; break;
                    case KeyCode.KeyD: right = value; break;
                }
            }
            else if (we.Event is MouseInput mi)
            {
                fire = mi.Button == MouseButton.Left && mi.State == InputState.Pressed;
            }
        }

        if (evt is DeviceEvent de && de.Event is MouseMotion mm)
        {
            Turn(-mm.Delta.X);
            Aim(mm.Delta.Y);
        }
    }
}
