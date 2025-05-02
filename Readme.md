# Calendly Clone API

A Rust-based REST API for a scheduling and calendar management system, similar to Calendly. Built with Actix-web and MongoDB.

## Features

- 👤 **User Management**
  - Registration with email verification
  - Login with JWT authentication
  - Password reset functionality
  - Token refresh mechanism

- 📅 **Calendar Management**
  - Create and manage calendar settings
  - Customize working hours
  - Set buffer times between meetings
  - Configure timezone and time format preferences

## Tech Stack

- **Framework**: Actix-web
- **Database**: MongoDB
- **Authentication**: JWT (JSON Web Tokens)
- **Email Service**: SMTP
- **Validation**: Validator
- **Password Hashing**: Bcrypt

## Project Structure

```
src/
├── app.rs              # Application setup and configuration
├── config/             # Configuration management
├── errors/            # Error handling
├── middleware/        # Custom middleware (auth, etc.)
├── modules/           # Feature modules
│   ├── user/         # User management
│   └── calendar/     # Calendar management
├── services/         # External services (email, etc.)
└── utils/            # Utility functions
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- MongoDB
- SMTP email service

### Environment Setup

Create a `.env` file in the root directory with the following variables:

```env
MONGODB_URI=your_mongodb_connection_string
DATABASE_NAME=your_database_name
PORT=8080
RUST_LOG=debug
EMAIL_USER=your_email@example.com
EMAIL_PASSWORD=your_email_app_password
JWT_SECRET=your_jwt_secret
```

### Installation

1. Clone the repository:
```bash
git clone <repository-url>
cd calendly
```

2. Build the project:
```bash
cargo build
```

3. Run the server:
```bash
cargo run
```

## API Endpoints

### User Management

- `POST /api/users/register` - Register new user
- `POST /api/users/login` - User login
- `POST /api/users/verify-email` - Verify email
- `POST /api/users/refresh-token` - Refresh access token
- `POST /api/users/forgot-password` - Request password reset
- `POST /api/users/reset-password` - Reset password

### Calendar Management

- `POST /api/calendar/settings` - Create calendar settings
- `PUT /api/calendar/settings` - Update calendar settings
- `DELETE /api/calendar/settings` - Delete calendar settings

## Authentication

The API uses JWT for authentication. Include the token in the Authorization header:

```bash
Authorization: Bearer <your_access_token>
```

Access tokens are valid for 7 days. Refresh tokens can be used to obtain new access tokens.

## Development

### Running Tests
```bash
cargo test
```

### Code Formatting
```bash
cargo fmt
```

### Linting
```bash
cargo clippy
```

## Security Considerations

- JWT secrets should be strong and kept secure
- Email passwords should be app-specific passwords
- MongoDB connection strings should be properly secured
- All sensitive information should be stored in environment variables

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Actix-web framework
- MongoDB Rust driver
- All other open-source contributors

## Contact

Abraham Anavheoba - wisdomabraham92@gmail.com

Project Link: [https://github.com/anavheoba/calendly](https://github.com/anavheoba/calendly)
