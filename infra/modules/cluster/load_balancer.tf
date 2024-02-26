resource "aws_lb" "main" {
  name               = "cluster-lb"
  internal           = false
  load_balancer_type = "application"
  security_groups    = [aws_security_group.lb.id]
  subnets            = toset(var.subnet_ids.public[*])
}

resource "aws_lb_listener" "http" {
  count             = var.environment == "dev" ? 1 : 0
  load_balancer_arn = aws_lb.main.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type             = "forward"
    target_group_arn = aws_lb_target_group.public.arn
  }
}

resource "aws_lb_listener" "http_redirect" {
  count             = var.environment == "dev" ? 0 : 1
  load_balancer_arn = aws_lb.main.arn
  port              = 80
  protocol          = "HTTP"

  default_action {
    type = "redirect"
    redirect {
      port        = "443"
      protocol    = "HTTPS"
      status_code = "HTTP_301"
    }
  }
}

resource "aws_lb_target_group" "public" {
  name        = "${var.service}-${var.environment}-public"
  port        = 80
  protocol    = "HTTP"
  vpc_id      = var.vpc_id
  target_type = "ip"

  health_check {
    protocol = "HTTP"
    path     = "/"
    matcher  = "200-299"
  }
}

resource "aws_lb_target_group_attachment" "all" {
  for_each         = toset(local.targets)
  target_group_arn = aws_lb_target_group.public.arn
  target_id        = each.key
  port             = 80
}

locals {
  targets = flatten([aws_instance.bootstrap[*].id, aws_instance.workers[*].id, aws_instance.managers[*].id])

}
