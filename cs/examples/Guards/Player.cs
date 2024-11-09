using FyroxLite;

[Uuid("c5671d19-9f1a-4286-8486-add4ebaadaec")]
public class Player : NodeScript
{
    private float sensitivity;
    private Node camera;
    private float power;
    private Prefab bullet;
    private float initial_bullet_velocity;
    private float shooting_range;
    private float reload_delay_sec;

    [HideInInspector] [Transient] private float ReloadSec;

    [HideInInspector] [Transient] private bool Published;

    [HideInInspector] [Transient] private Node Collider;

    [HideInInspector] [Transient] private float AimY;

    private const int FractionPlayer = 0;

    protected override void OnInit()
    {
        Window.CursorGrab = CursorGrabMode.Confined;
        Collider = Node.FindColliderInChildren() ?? throw new Exception("player collider missing");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo();
    }

    protected override void OnMessage(object message)
    {
        if (message is BulletHitMessage hitMessage && hitMessage.Fraction != FractionPlayer)
        {
            Plugin.Get<Game>().IncWounds();
            Console.WriteLine("player wounded!");
        }
    }

    protected override void OnUpdate(float dt)
    {
        if (ReloadSec > 0.0f)
        {
            ReloadSec -= dt;
        }

        if (!Published)
        {
            Published = true;
            Plugin.Get<Game>().player = Node;
        }

        if (Input.IsMouseButtonPressed(Input.MouseLeft))
        {
            if (ReloadSec <= 0.0f)
            {
                ReloadSec = reload_delay_sec;
                Fire();
            }
        }

        var moveDelta = Vector3.Zero;

        if (Input.IsKeyPressed(KeyCode.W))
        {
            moveDelta.Z += 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.S))
        {
            moveDelta.Z -= 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.A))
        {
            moveDelta.X += 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.D))
        {
            moveDelta.X -= 1.0f;
        }

        Turn(-Input.MouseMove.X);
        Aim(Input.MouseMove.Y);

        if (moveDelta.Length() > Mathf.Epsilon)
        {
            moveDelta = moveDelta.Normalized();
        }

        Node.AsRigidBody().Value.ApplyForce(Node.LocalRotation * moveDelta * power);
    }

    private void Turn(float x)
    {
        Node.LocalRotation *= new Quaternion(Vector3.Up, sensitivity * x);
    }

    private void Aim(float y)
    {
        AimY += y * sensitivity;
        AimY = Mathf.Clamp(AimY, -MathF.PI / 2.0f, MathF.PI / 2.0f);

        camera.LocalRotation = new Quaternion(Vector3.Left, AimY);
    }

    private void Fire()
    {
        Bullet.Spawn(new Bullet.BulletSeed
        {
            Prefab = bullet,
            Origin = camera.GlobalPosition,
            Direction = camera.GlobalRotation * Vector3.Forward,
            InitialVelocity = initial_bullet_velocity,
            AuthorCollider = Collider,
            Range = shooting_range,
            Fraction = FractionPlayer,
        });
    }
}