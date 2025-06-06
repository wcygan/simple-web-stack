import { PageProps } from "fresh";
import RegisterForm from "../islands/RegisterForm.tsx";

export default function RegisterPage(props: PageProps) {
  return (
    <div class="min-h-screen bg-gray-50 flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
        <div class="max-w-md w-full space-y-8">
          <div>
            <h1 class="text-center text-3xl font-extrabold text-gray-900">
              Simple Todo App
            </h1>
            <p class="mt-2 text-center text-sm text-gray-600">
              Create your account to get started with managing your tasks.
            </p>
          </div>
          <RegisterForm />
        </div>
      </div>
  );
}